use mlua::prelude::*;
use mlua::Lua;
use mlua::UserData;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

mod util;
pub use util::{cache, load, FileName, CACHE_DEV_FILE, CACHE_REPO_FILE};
// use tracing::info;

const DATA_EXPIRED: &str = "Data expired";
const NO_DATA: &str = "No data";

trait Trending: Debug + Default {
    /// `parse_html` is the process to grap info from
    /// github http trending api response html.
    fn parse_html(content: String) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Collaborator {
    pub name: String,
    pub avatar: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub author: String,
    pub name: String,
    pub link: String,
    pub description: String,
    pub star_count: u32,
    pub add: String,
    pub forks: u32,
    pub language: String,
    pub build_by: Vec<Collaborator>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Developer {
    pub name: String,
    pub avatar: String,
    pub popular_repo: String,
    pub description: String,
}
impl UserData for Collaborator {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("avatar", |_, this| Ok(this.avatar.clone()));
    }
}

impl UserData for Repository {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("author", |_, this| Ok(this.author.clone()));
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("link", |_, this| Ok(this.link.clone()));
        fields.add_field_method_get("description", |_, this| Ok(this.description.clone()));
        fields.add_field_method_get("star_count", |_, this| Ok(this.star_count));
        fields.add_field_method_get("add", |_, this| Ok(this.add.clone()));
        fields.add_field_method_get("forks", |_, this| Ok(this.forks));
        fields.add_field_method_get("language", |_, this| Ok(this.language.clone()));
        fields.add_field_method_get("build_by", |_, this| Ok(this.build_by.clone()));
    }
}

impl UserData for Developer {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("avatar", |_, this| Ok(this.avatar.clone()));
        fields.add_field_method_get("popular_repo", |_, this| Ok(this.popular_repo.clone()));
        fields.add_field_method_get("description", |_, this| Ok(this.description.clone()));
    }
}

#[derive(Debug, Clone)]
pub enum UserDataType {
    Repository(Repository),
    Developer(Developer),
}

impl UserData for UserDataType {}

impl Trending for Repository {
    fn parse_html(content: String) -> Vec<Self> {
        let doucument = Html::parse_document(&content);
        let p_selector = Selector::parse(r"p").unwrap();
        let a_selector = Selector::parse(r"a").unwrap();
        let img_selector = Selector::parse(r"img").unwrap();
        let div_selector = Selector::parse(r"div").unwrap();
        let span_selector = Selector::parse(r"span").unwrap();
        let h2_selector = Selector::parse(r#"h2[class="h3 lh-condensed"]"#).unwrap();
        let article_selector = Selector::parse(r#"article[class="Box-row"]"#).unwrap();
        let program_span_sel = Selector::parse(r#"span[itemprop="programmingLanguage"]"#).unwrap();

        let mut repos: Vec<Repository> = Vec::new();
        let mut url: String = "https://github.com".to_string();

        for per_repo in doucument.select(&article_selector) {
            let mut repo = Self::default();
            assert_eq!(per_repo.value().name(), "article");
            if let Some(p) = per_repo.select(&p_selector).next() {
                repo.description = p
                    .text()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|x| x.to_string().trim().to_string())
                    .collect();
            }

            let a_link = per_repo
                .select(&h2_selector)
                .next()
                .unwrap()
                .select(&a_selector)
                .next()
                .unwrap();

            let repo_link = a_link.value().attr("href").unwrap();
            url.push_str(repo_link);
            repo.link.clone_from(&url);
            url = url.replace(repo_link, "");

            let tmp = a_link
                .text()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|e| e.to_string().trim().to_owned())
                .collect::<String>()
                .clone();

            let name_vec = tmp.split(' ').collect::<Vec<_>>();
            repo.author = name_vec[0].to_string();
            repo.name = name_vec[1].to_string().replace('/', "");

            let div = per_repo.select(&div_selector).nth(2).unwrap();
            if let Some(span) = div.select(&program_span_sel).next() {
                repo.language = span.text().collect();
            }

            let mut repo_link = repo_link.to_owned();
            repo_link.push_str("/stargazers");

            let mut attr = "a[href=\"".to_string();
            attr.push_str(&repo_link);
            attr.push_str("\"]");

            let start_a_sel = Selector::parse(&attr).unwrap();
            let star_count = div
                .select(&start_a_sel)
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            repo.star_count = star_count
                .replace(',', "")
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            attr = attr.replace("/stargazers", "/forks");
            let fork_a_sel = Selector::parse(&attr).unwrap();
            let fork_count = div
                .select(&fork_a_sel)
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            repo.forks = fork_count.replace(',', "").trim().parse().unwrap();

            let mut spans: Vec<_> = per_repo
                .select(&span_selector)
                .filter(|span| span.value().attr("itemprop").is_none())
                .collect();

            spans.reverse();
            repo.add = spans
                .first()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let mut collaborators = vec![];

            for col_a_link in spans[1].select(&a_selector) {
                let mut collaborator = Collaborator::default();
                let col_avator_img = col_a_link.select(&img_selector).next().unwrap();
                collaborator.name = col_avator_img
                    .value()
                    .attr("alt")
                    .unwrap()
                    .split('@')
                    .collect();

                collaborator.avatar = col_avator_img.value().attr("src").unwrap().to_string();

                collaborators.push(collaborator);
            }
            repo.build_by = collaborators;

            repos.push(repo);
        }

        repos
    }
}

impl Trending for Developer {
    fn parse_html(content: String) -> Vec<Self> {
        let document = Html::parse_document(&content);
        let article_sel = Selector::parse(r#"article[class="Box-row d-flex"]"#).unwrap();
        let art_sel = Selector::parse(r"article").unwrap();
        let div_sel = Selector::parse(r"div").unwrap();
        let img_sel = Selector::parse(r"img").unwrap();
        let h1_sel = Selector::parse(r"h1").unwrap();
        let a_sel = Selector::parse(r"a").unwrap();

        let mut developers = vec![];
        let mut url = "https://github.com".to_string();

        for article in document.select(&article_sel) {
            let mut dev = Self {
                avatar: article
                    .select(&img_sel)
                    .next()
                    .unwrap()
                    .value()
                    .attr("src")
                    .unwrap()
                    .to_string(),
                name: article
                    .select(&h1_sel)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string(),
                ..Default::default()
            };

            if let Some(s) = article.select(&art_sel).next() {
                if let Some(h1) = s.select(&h1_sel).next() {
                    let href = h1
                        .select(&a_sel)
                        .next()
                        .unwrap()
                        .value()
                        .attr("href")
                        .unwrap();
                    url.push_str(href);
                    dev.popular_repo = url.to_string();
                    url = url.replace(href, "");
                }
            }

            if let Some(s) = article.select(&art_sel).next() {
                if let Some(div) = s.select(&div_sel).nth(1) {
                    dev.description = div.text().collect::<String>().trim().to_string();
                }
            }
            developers.push(dev);
        }

        developers
    }
}

fn init_client(proxy: Option<String>) -> Result<reqwest::Client, Box<dyn Error>> {
    if let Some(pro_str) = proxy {
        let pro = reqwest::Proxy::all(pro_str)?;
        let client = reqwest::Client::builder().proxy(pro).build()?;
        Ok(client)
    } else {
        let client = reqwest::Client::builder().build()?;
        Ok(client)
    }
}
/// process_repo represents the process of get repositories.
#[tokio::main]
pub async fn process_repo() -> Result<Vec<UserDataType>, Box<dyn Error>> {
    let f = FileName::CacheRepoFile(CACHE_REPO_FILE);
    let lua = Lua::new();
    let data = match load(f).await {
        Ok(data) => data,
        Err(err) => {
            if err.to_string() != DATA_EXPIRED && err.to_string() != NO_DATA {
                lua.globals().set("err_info", err.to_string()).unwrap();
                lua.load(r"# print(vim.notify(err_info))").exec().unwrap();
            }
            // tracing_subscriber::fmt::init();
            let client = init_client(None)?;
            let res = client.get("https://github.com/trending").send().await?;
            assert!(res.status().is_success());
            let text = res.text().await?;
            let repos = Repository::parse_html(text);
            // let repo_json = serde_json::to_string_pretty(&repos).unwrap();
            // info!("{repo_json}");
            let repos = serde_json::to_value(repos.clone())?;
            cache(repos.clone(), f).await?;
            repos
        }
    };

    let data: Vec<Repository> = serde_json::from_value(data)?;
    let result: Vec<UserDataType> = data.into_iter().map(UserDataType::Repository).collect();
    return Ok(result);
}

/// process_devloper represents the process of get developers.
#[tokio::main]
pub async fn process_devloper() -> Result<Vec<UserDataType>, Box<dyn Error>> {
    let f = FileName::CacheDevFile(CACHE_DEV_FILE);
    let lua = Lua::new();
    let data = match load(f).await {
        Ok(data) => data,
        Err(err) => {
            if err.to_string() != DATA_EXPIRED && err.to_string() != NO_DATA {
                lua.globals().set("err_info", err.to_string()).unwrap();
                lua.load(r"# print(vim.notify(err_info))").exec().unwrap();
            }
            let client = init_client(None)?;
            let res = client
                .get("https://github.com/trending/developers")
                .send()
                .await?;
            assert!(res.status().is_success());
            let text = res.text().await?;
            let developers = Developer::parse_html(text);
            // let developer_json = serde_json::to_string_pretty(&developers).unwrap();
            // info!("{developer_json}");
            let developers = serde_json::to_value(developers)?;
            cache(developers.clone(), f).await?;
            developers
        }
    };

    let data: Vec<Developer> = serde_json::from_value(data)?;
    let result: Vec<UserDataType> = data.into_iter().map(UserDataType::Developer).collect();
    return Ok(result);
}

fn process<F>(lua: &Lua, f: F) -> LuaResult<LuaTable>
where
    F: Fn() -> Result<Vec<UserDataType>, Box<dyn std::error::Error>>,
{
    let datas = f()
        .map_err(|e| mlua::Error::RuntimeError(format!("Error: {e}")))
        .unwrap();
    let array_table = lua.create_table()?;

    for (i, data) in datas.into_iter().enumerate() {
        match data {
            UserDataType::Repository(repo) => array_table.set(i + 1, repo)?,
            UserDataType::Developer(dev) => array_table.set(i + 1, dev)?,
        };
    }
    Ok(array_table)
}

#[mlua::lua_module]
fn ghtrending_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let process_devloper = lua.create_function(|lua, ()| {
        process(lua, process_devloper).map_err(|err| mlua::Error::RuntimeError(err.to_string()))
    })?;
    let process_repo = lua.create_function(|lua, ()| {
        process(lua, process_repo).map_err(|err| mlua::Error::RuntimeError(err.to_string()))
    })?;

    exports.set("process_developer", process_devloper)?;
    exports.set("process_repo", process_repo)?;
    Ok(exports)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mlua::{chunk, Lua, Result};
//     #[test]
//     fn test_repository_state() -> Result<()> {
//         // create a lua state
//         let lua = Lua::new();
//         let repo = Repository {
//             name: "test".into(),
//             ..Default::default()
//         };
//         lua.load(chunk! {
//             local rep = $repo
//             assert(rep.name == "test")
//         })
//         .exec()
//     }
// }
