use super::{ObjResponse, ObjResult};

impl ObjResponse {
    pub fn unwrap(&self) -> Result<(), (u16, String)> {
        match self.success {
            true => Ok(()),
            false => Err((
                500,
                self.errors
                    .iter()
                    .map(|each| each.0.message.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n"),
            )),
        }?;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        match self.result.as_ref() {
            Some(objresult) => match objresult {
                ObjResult::ZonesData(vec_zone) => vec_zone.is_empty(),
                ObjResult::DNSRecords(vec_rec) => vec_rec.is_empty(),
                _ => unreachable!()
                // ObjResult::ZoneData(_) => unreachable!(),
                // ObjResult::DNSRecord(_) => unreachable!(),
            }
            None => true
        }
    }
}
