use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "orbyn-node",
    about = "Nodo de ejecución local o remoto de ORBYN"
)]
struct Args {
    #[arg(long, env = "ORBYN_CONTROL_URL")]
    control_url: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    tracing::info!(control_url = ?args.control_url, "Nodo ORBYN iniciado");
    tracing::warn!("El enrolamiento y la ejecución de tareas todavía no están implementados");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
