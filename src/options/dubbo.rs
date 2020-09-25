use structopt::StructOpt;
use tokio::process::Command;

#[derive(StructOpt, Debug)]
pub struct DubboOpts {
    #[structopt(
        name = "active-maven-profile",
        short,
        long,
        env = "ACTIVE_MAVEN_PROFILE"
    )]
    active_maven_profile: Option<String>,

    #[structopt(
        name = "from-image",
        short,
        long,
        default_value = "registry.cn-shanghai.aliyuncs.com/qingmuio/openjre:latest",
        env = "FROM_IMAGE"
    )]
    from_image: String,

    #[structopt(name = "image", short, long, env = "IMAGE")]
    image: String,

    #[structopt(name = "module-name", short, long, env = "MODULE_NAME")]
    module_name: String,

    #[structopt(name = "username", short, long, env = "DOCKER_USERNAME")]
    user_name: Option<String>,

    #[structopt(name = "password", short, long, env = "DOCKER_PASSWORD")]
    password: Option<String>,
}

impl DubboOpts {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut mvn_command = Command::new("mvn");

        if let Some(active_maven_profile) = &self.active_maven_profile {
            mvn_command.arg(format!("-P{}", &active_maven_profile));
        }

        if let Some(user_name) = &self.user_name {
            if let Some(password) = &self.password {
                mvn_command.arg(format!("-Djib.to.auth.username={}", &user_name));
                mvn_command.arg(format!("-Djib.to.auth.password={}", &password));
            }
        }

        mvn_command
            .arg(format!("-Djib.from.image={}", &self.from_image))
            .arg(format!("-Dimage={}", &self.image))
            .args(&["-pl", &self.module_name, "-am"])
            .spawn()?
            .await?;

        Ok(())
    }
}
