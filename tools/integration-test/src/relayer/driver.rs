use ibc_relayer::chain::handle::ProdChainHandle;
use ibc_relayer::config::SharedConfig;
use ibc_relayer::registry::SharedRegistry;
use ibc_relayer::supervisor::{spawn_supervisor, SupervisorHandle, SupervisorOptions};
use std::path::PathBuf;

use crate::error::Error;
use crate::types::env::{EnvWriter, ExportEnv};
use crate::util::suspend::hang_on_error;

#[derive(Clone)]
pub struct RelayerDriver {
    /**
       The path to the relayer config saved on the filesystem.

       This allows users to test the relayer manually with the config file
       while the test is suspended.
    */
    pub config_path: PathBuf,

    /**
       The relayer [`Config`](ibc_relayer::config::Config) that is shared
       with the [`Registry`](ibc_relayer::registry::Registry).

       Use this shared config when spawning new supervisor using
       [`spawn_supervisor`](ibc_relayer::supervisor::spawn_supervisor).
    */
    pub config: SharedConfig,

    /**
       The relayer chain [`Registry`](ibc_relayer::registry::Registry)
       that is shared with any running supervisor.

       Use this shared registry when spawning new supervisor using
       [`spawn_supervisor`](ibc_relayer::supervisor::spawn_supervisor).
    */
    pub registry: SharedRegistry<ProdChainHandle>,

    pub hang_on_fail: bool,
}

impl RelayerDriver {
    pub fn spawn_supervisor(&self) -> Result<SupervisorHandle, Error> {
        spawn_supervisor(
            self.config.clone(),
            self.registry.clone(),
            None,
            SupervisorOptions {
                health_check: false,
                force_full_scan: false,
            },
        )
        .map_err(Error::supervisor)
    }

    pub fn with_supervisor<R>(&self, cont: impl FnOnce() -> Result<R, Error>) -> Result<R, Error> {
        let _handle = self.spawn_supervisor()?;

        cont().map_err(hang_on_error(self.hang_on_fail))
    }
}

impl ExportEnv for RelayerDriver {
    fn export_env(&self, writer: &mut impl EnvWriter) {
        writer.write_env("RELAYER_CONFIG", &format!("{}", self.config_path.display()));
    }
}
