// Copyright (c) 2022 - 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use super::{
    response::Tpm2ResponseHeader, TPM2_COMMAND_HEADER_SIZE, TPM2_RESPONSE_HEADER_SIZE,
    TPM_RC_SUCCESS, TPM_SHUTDOWN_CMD,
};
use crate::execute_command;
use core::convert::TryFrom;
use global::{VtpmError, VtpmResult, VTPM_MAX_BUFFER_SIZE};

pub fn tpm2_shutdown() -> VtpmResult {
    let mut tpm_rsp: [u8; VTPM_MAX_BUFFER_SIZE] = [0; VTPM_MAX_BUFFER_SIZE];

    let _ = execute_command(&TPM_SHUTDOWN_CMD, &mut tpm_rsp, 0);
    let mut buf: [u8; TPM2_COMMAND_HEADER_SIZE] = [0; TPM2_COMMAND_HEADER_SIZE];
    buf.copy_from_slice(&tpm_rsp[..TPM2_RESPONSE_HEADER_SIZE]);
    let rsp = Tpm2ResponseHeader::try_from(buf);
    if rsp.is_err() {
        log::error!("Invalid Tpm2ResponseHeader!\n");
        log::error!("  {:02x?}\n", &buf);
        return Err(VtpmError::TpmLibError);
    }

    let rsp = rsp.unwrap();
    if rsp.response_code != TPM_RC_SUCCESS {
        log::error!("Tpm2PcrExtend failed.\n");
        return Err(VtpmError::TpmLibError);
    }

    Ok(())
}
