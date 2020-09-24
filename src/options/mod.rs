use once_cell::sync::Lazy;
use structopt::StructOpt;

pub mod dubbo;
pub mod go;
pub mod springboot;
pub mod tomcat;

pub static OPTIONS: Lazy<Options> = Lazy::new(Options::from_args);

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rgbt",
    about = "rust gd build tools",
    author = "goudai-project <goudai@goudai.io>",
    after_help = "source: https://github.com/goudai-projects/rgbt"
)]
pub struct Options {
    #[structopt(long)]
    pub verbose: bool,

    #[structopt(flatten)]
    pub build: BuildOptions,
}

#[derive(StructOpt, Debug)]
pub enum BuildOptions {
    #[structopt(name = "go", about = "Build a go project to docker image")]
    GoOpts(go::GoOpts),

    #[structopt(
        name = "dubbo",
        about = "Build multi-module dubbo project to docker image"
    )]
    DubboOpts(dubbo::DubboOpts),

    #[structopt(
        name = "springboot",
        about = "Build a spring boot application to docker image "
    )]
    SpringBootOpts(springboot::SpringBootOpts),

    #[structopt(name = "tomcat", about = "Build A tomcat project to docker image")]
    TomcatOpts(tomcat::TomcatOpts),
}
