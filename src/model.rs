#[derive(Debug)]
pub enum Permission {
    ReadChannel,   // 1 << 0
    CreateChannel, // 1 << 1
    UpdateChannel, // 1 << 2
}

impl Permission {
    pub fn from_bits(bits: u8) -> Vec<Permission> {
        let mut permissions = Vec::new();
        if bits & 1 == 1 {
            permissions.push(Permission::ReadChannel);
        }
        if bits & 2 == 2 {
            permissions.push(Permission::CreateChannel);
        }
        if bits & 4 == 4 {
            permissions.push(Permission::UpdateChannel);
        }
        permissions
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            Permission::ReadChannel => 1,
            Permission::CreateChannel => 2,
            Permission::UpdateChannel => 4,
        }
    }
}
