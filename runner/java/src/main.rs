use std::path::PathBuf;
use bollard::Docker;

use runner::Runner;

struct Java {
    docker: Docker,
}

impl Runner for Java {
    fn run(&self, artifact: &runner::Artifact, input: &str) -> runner::Result<String> {
        let path = artifact.download()?;

        // let output = self.run(path, input)?;
        
        todo!();
    }
}

fn main() -> runner::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let java = Java {
        docker,
    };
    Ok(())
}
