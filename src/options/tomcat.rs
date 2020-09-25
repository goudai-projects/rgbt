use structopt::StructOpt;
use tokio::process::Command;

#[derive(StructOpt, Debug)]
pub struct TomcatOpts {
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
        default_value = "registry.cn-shanghai.aliyuncs.com/qingmuio/tomcat:9.0.37",
        env = "FROM_IMAGE"
    )]
    from_image: String,

    #[structopt(name = "image", short, long, env = "IMAGE")]
    image: String,

    #[structopt(name = "username", short, long, env = "DOCKER_USERNAME")]
    user_name: Option<String>,

    #[structopt(name = "password", short, long, env = "DOCKER_PASSWORD")]
    password: Option<String>,

    #[structopt(
        name = "app-root",
        long,
        env = "APP_ROOT",
        default_value = "/usr/local/tomcat/webapps/ROOT"
    )]
    app_root: String,
}

impl TomcatOpts {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut tomcat_command = Command::new("mvn");
        tomcat_command.args(&[
            "clean",
            "package",
            "com.google.cloud.tools:jib-maven-plugin:build",
        ]);

        if let Some(active_maven_profile) = &self.active_maven_profile {
            tomcat_command.arg(format!("-P {}", &active_maven_profile));
        }

        if let Some(user_name) = &self.user_name {
            if let Some(password) = &self.password {
                tomcat_command.arg(format!("-Djib.to.auth.username={}", &user_name));
                tomcat_command.arg(format!("-Djib.to.auth.password={}", &password));
            }
        }

        tomcat_command
            .arg(format!("-Djib.from.image={}", &self.from_image))
            .arg(format!("-Dimage={}", &self.image))
            .arg(format!("-Djib.container.appRoot={}", &self.app_root))
            .spawn()?
            .await?;

        Ok(())
    }
}
