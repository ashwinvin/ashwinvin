from typing import List

import httpx
from starlite import Provide, Starlite, StaticFilesConfig, Template, TemplateConfig, get
from starlite.template import JinjaTemplateEngine


@get(path="/")
async def index(repos: List[str]) -> Template:
    data = {"repos": {}}
    async with httpx.AsyncClient() as client:
        for repo in repos:
            response = await client.get(f"https://api.github.com/repos/{repo}")
            print(response.status_code)
            repo_data = response.json()
            data["repos"].update(
                {
                    repo_data["name"]: {
                        "stars": repo_data["stargazers_count"],
                        "description": repo_data["description"],
                        "link": repo_data["url"],
                    }
                }
            )
    return Template(name="index.html", context=data)


def get_config() -> List[str]:
    return ["ashwinvin/visionlib"]


app = Starlite(
    route_handlers=[index],
    static_files_config=[StaticFilesConfig(directories=["../frontend"], path="/static")],
    template_config=TemplateConfig(directory="../frontend", engine=JinjaTemplateEngine),
    dependencies={"repos": Provide(get_config)},
)
