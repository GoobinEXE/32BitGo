use bitgo_core::{
    validate_config_payload, Capabilities, CoreError, CoreResult, DeviceDriver, DeviceIdentity,
    DeviceKind, DeviceStatus, EndpointClass, HidTransport, KeyboardEdition, PayloadGate, Profile,
    ProfileSlot, ThemeHint,
};

pub struct RetroKeyboardDriver {
    identity: DeviceIdentity,
    gate: PayloadGate,
}

impl RetroKeyboardDriver {
    pub fn new(mut identity: DeviceIdentity) -> Self {
        if identity.edition.is_none() {
            identity.edition = Some(KeyboardEdition::Nes);
        }
        identity.kind = DeviceKind::RetroKeyboard;
        Self {
            identity,
            gate: PayloadGate {
                max_len: 64,
                allowed_report_ids: &[],
            },
        }
    }

    fn ensure_config_write(&self, data: &[u8]) -> CoreResult<()> {
        validate_config_payload(EndpointClass::Config, data, &self.gate)
    }
}

impl DeviceDriver for RetroKeyboardDriver {
    fn identity(&self) -> &DeviceIdentity {
        &self.identity
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            remapping: true,
            macros: true,
            super_buttons: true,
            profiles: true,
            battery: true,
            theme_hint: ThemeHint::Nes,
        }
    }

    fn read_status(&mut self, _transport: &mut dyn HidTransport) -> CoreResult<DeviceStatus> {
        Err(CoreError::Protocol(
            "HID status report map pending reverse-engineering fixtures".into(),
        ))
    }

    fn pull_profile(
        &mut self,
        _transport: &mut dyn HidTransport,
        _slot: ProfileSlot,
    ) -> CoreResult<Profile> {
        Err(CoreError::Protocol(
            "profile pull pending reverse-engineering fixtures".into(),
        ))
    }

    fn push_profile(
        &mut self,
        transport: &mut dyn HidTransport,
        profile: &Profile,
    ) -> CoreResult<()> {
        let _ = (transport, profile);
        Err(CoreError::Protocol(
            "profile push pending reverse-engineering fixtures; use dry-run encode path only"
                .into(),
        ))
    }
}

impl RetroKeyboardDriver {
    pub fn encode_profile_dry_run(&self, profile: &Profile) -> CoreResult<Vec<u8>> {
        let mut frame = vec![0u8; 8];
        frame[0] = 0x00;
        frame[1] = profile.slot.0;
        frame[2] = profile.keys.len().min(255) as u8;
        frame[3] = profile.macros.len().min(255) as u8;
        frame[4] = profile.super_buttons.len().min(255) as u8;
        self.ensure_config_write(&frame)?;
        if self.gate.allowed_report_ids.is_empty() {
            return Err(CoreError::ValidationRejected(
                "report-id allowlist empty until USB capture lands; refusing encode".into(),
            ));
        }
        Ok(frame)
    }
}
