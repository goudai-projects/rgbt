use structopt::StructOpt;
use tokio::{fs, process::Command};

const DOCKER_FILE: &str = r#"
FROM golang:{version} as builder
WORKDIR /workspace
COPY . .
ENV GOPROXY=https://goproxy.io
RUN ls . && go mod download &&  CGO_ENABLED=0 GOOS=linux GOARCH=amd64 GO111MODULE=on go build -a -o app main.go


FROM registry.cn-shanghai.aliyuncs.com/qingmuio/distroless_static:nonroot
WORKDIR /
COPY --from=builder /workspace/manager .
USER nonroot:nonroot
ENTRYPOINT ["/app"]
"#;

#[derive(StructOpt, Debug)]
pub struct GoOpts {
    #[structopt(
        name = "version",
        short,
        long,
        default_value = "1.15.1",
        env = "GO_DOCKERFILE_VERSION"
    )]
    version: String,

    #[structopt(name = "image", short, long, env = "IMAGE")]
    image: String,
}

impl GoOpts {
    pub async fn run(&self) -> anyhow::Result<()> {
        let dockerfile: String = DOCKER_FILE.replace("{version}", &self.version);

        fs::write("Dockerfile", dockerfile).await?;

        Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&self.image)
            .arg(".")
            .spawn()?
            .await?;

        Ok(())
    }
}
