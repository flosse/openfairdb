use crate::core::prelude::*;

pub fn confirm_email_address(db: &dyn Db, token: &str) -> Result<()> {
    let email_nonce =
        EmailNonce::decode_from_str(token).map_err(|_| ParameterError::TokenInvalid)?;
    let mut user = db.get_user_by_email(&email_nonce.email)?;
    if !user.email_confirmed {
        user.email_confirmed = true;
        debug_assert_eq!(Role::Guest, user.role);
        if user.role == Role::Guest {
            user.role = Role::User;
        }
        db.update_user(&user)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::MockDb;
    use super::*;

    #[test]
    fn confirm_email_of_existing_user() {
        let db = MockDb::default();
        let email = "a@foo.bar";
        db.users.borrow_mut().push(User {
            email: email.into(),
            email_confirmed: false,
            password: "secret".parse::<Password>().unwrap(),
            role: Role::Guest,
        });
        let email_nonce = EmailNonce {
            email: email.into(),
            nonce: Nonce::new(),
        };
        assert!(confirm_email_address(&db, &email_nonce.encode_to_string()).is_ok());
        assert_eq!(db.users.borrow()[0].email_confirmed, true);
        assert_eq!(db.users.borrow()[0].role, Role::User);
    }
}
