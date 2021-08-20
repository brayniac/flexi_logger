// Copyright 2021 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::writers::LogWriter;

/// No-op Writer
pub struct NoopWriter {}

impl LogWriter for NoopWriter {
    fn write(
        &self,
        _: &mut crate::DeferredNow,
        _: &crate::Record<'_>,
    ) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
    fn flush(&self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
    fn max_log_level(&self) -> crate::LevelFilter {
        crate::LevelFilter::Off
    }
}
