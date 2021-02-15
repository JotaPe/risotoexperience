use rusty_money::{iso, Money};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};
use uuid::Uuid;
use validator::{validate_email, validate_phone};
use rayon::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct User {
    pub user_id: String, // UUID v4
    pub email: String,
    pub phone: String,
    pub address: String,
    pub image_url: String,
    pub password_hash: String,
    pub confirmed: bool,
    pub roles: Vec<String>,
}

impl User {
    pub fn new(
        user_id: &str,
        email: &str,
        phone: &str,
        address: &str,
        image_url: &str,
        password_hash: &str,
        confirmed: bool,
        roles: Vec<&str>,
    ) -> User {
        User {
            user_id: user_id.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            image_url: image_url.to_string(),
            password_hash: password_hash.to_string(),
            confirmed,
            roles: roles.par_iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(
        user_id: &str,
        email: &str,
        phone: &str,
        address: &str,
        image_url: &str,
        password_hash: &str,
        confirmed: bool,
        roles: Vec<&str>,
    ) -> Result<User, &'static str> {
        match Uuid::parse_str(user_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("UUID is not valid"),
        };
        match validate_email(email) {
            true => true,
            false => return Err("Email is not valid"),
        };
        match validate_phone(phone) {
            true => true,
            false => return Err("Phone is invalid"),
        };
        match Url::parse(image_url) {
            Ok(url) => url,
            Err(_) => return Err("URL is not valid"),
        };
        Ok(User::new(
            user_id,
            email,
            phone,
            address,
            image_url,
            password_hash,
            confirmed,
            roles,
        ))
    }

    pub fn update_email(&self, email: &str) -> Result<User, &'static str> {
        match validate_email(email) {
            true => true,
            false => return Err("Email is not valid"),
        };
        Ok(User {
            email: email.to_string(),
            ..self.clone()
        })
    }

    // not tested functions
    pub fn update_phone(&self, phone: &str) -> Result<User, &'static str> {
        match validate_phone(phone) {
            true => true,
            false => return Err("Phone number is not valid"),
        };
        Ok(User {
            phone: phone.to_string(),
            ..self.clone()
        })
    }

    // not tested functions
    pub fn update_address(&self, address: &str) -> Result<User, &'static str> {
        Ok(User {
            address: address.to_string(),
            ..self.clone()
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Product {
    pub product_id: String,  // UUID v4
    pub business_id: String, // UUID v4
    pub title: String,
    pub description: String,
    pub image_url: String,
    // We're storing as a String to be more usable in the future
    pub price: String,
    // unformatted_price is just to store the original input
    // to be used on the future for making a better formatted
    // version of price.
    pub unformatted_price: String,
    pub product_tags: Vec<String>,
}

impl Product {
    pub fn new(
        product_id: &str,
        business_id: &str,
        title: &str,
        description: &str,
        image_url: &str,
        price: &str,
        unformatted_price: &str,
        product_tags: Vec<&str>,
    ) -> Product {
        Product {
            product_id: product_id.to_string(),
            business_id: business_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            price: price.to_string(),
            unformatted_price: unformatted_price.to_string(),
            product_tags: product_tags.par_iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(
        product_id: &str,
        business_id: &str,
        title: &str,
        description: &str,
        image_url: &str,
        unformatted_price: &str,
        product_tags: Vec<&str>,
    ) -> Result<Product, &'static str> {
        match Uuid::parse_str(product_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("UUID is not valid"),
        };
        match Uuid::parse_str(business_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("Business UUID is not valid"),
        };
        match Url::parse(image_url) {
            Ok(url) => url,
            Err(_) => return Err("URL is not valid"),
        };
        if title.len() < 5 {
            return Err("Too short title");
        } else if title.len() > 100 {
            return Err("Too big title");
        }
        if description.len() < 5 {
            return Err("Too short description");
        } else if title.len() > 1000 {
            return Err("Too big description");
        }
        let price = Money::from_str(unformatted_price, iso::BRL)
            .unwrap()
            .to_string();
        Ok(Product::new(
            product_id,
            business_id,
            title,
            description,
            image_url,
            &price,
            unformatted_price,
            product_tags,
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Business {
    pub business_id: String,      // UUID v4
    pub user_id: String,          // UUID v4
    pub products_id: Vec<String>, // Vector of UUIDs v4
    pub business_tags: Vec<String>,
}

impl Business {
    pub fn new(
        business_id: &str,
        user_id: &str,
        products_id: Vec<&str>,
        business_tags: Vec<&str>,
    ) -> Business {
        Business {
            business_id: business_id.to_string(),
            user_id: user_id.to_string(),
            products_id: products_id.par_iter().map(|&val| val.to_string()).collect(),
            business_tags: business_tags.par_iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(
        business_id: &str,
        user_id: &str,
        products_id: Vec<&str>,
        business_tags: Vec<&str>,
    ) -> Result<Business, &'static str> {
        match Uuid::parse_str(business_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("A ID is not a UUID"),
        };
        match Uuid::parse_str(user_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("A ID is not a UUID"),
        };
        for id in &products_id {
            match Uuid::parse_str(id) {
                Ok(uuid_id) => uuid_id,
                Err(_) => return Err("A ID is not a UUID"),
            };
        }
        Ok(Business::new(
            business_id,
            user_id,
            products_id,
            business_tags,
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Order {
    pub order_id: String,   // UUID v4
    pub product_id: String, // UUID v4
    pub ordered_date: String,
    pub expected_date: String,
}

#[cfg(test)]
mod tests {
    use super::Business;
    use super::Product;
    use super::User;

    #[test]
    fn test_new_user() {
        let user = User::new(
            "XD",
            "email",
            "3091u2304",
            "dominguinhos",
            "http://image.url.com",
            "XDXD",
            false,
            vec!["admin"],
        );
        assert_eq!(
            User {
                user_id: "XD".to_string(),
                email: "email".to_string(),
                phone: "3091u2304".to_string(),
                address: "dominguinhos".to_string(),
                image_url: "http://image.url.com".to_string(),
                password_hash: "XDXD".to_string(),
                confirmed: false,
                roles: vec!["admin".to_string()],
            },
            user,
            "User new function is working correctly"
        )
    }

    #[test]
    fn test_create_user() {
        let user = User::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "email@example.com",
            "+5521965237969",
            "Rua Dominguinhos",
            "http://image.url.com",
            "XDXD",
            false,
            vec!["admin"],
        )
        .is_ok();
        assert!(user, "User create function is working correctly");
    }

    #[test]
    fn update_email_user() {
        let user = User::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "email@example.com",
            "+5521965237969",
            "Rua Dominguinhos",
            "http://image.url.com",
            "XDXD",
            false,
            vec!["admin"],
        )
        .unwrap();
        let updated_user = user.update_email("email@example2.com");
        assert!(updated_user.is_ok(), "User email updated successfully")
    }

    #[test]
    fn test_new_product() {
        let product = Product::new(
            "09320481032",
            "102938409128305",
            "aiowjefaieorf",
            "oiewroaer",
            "oijeaij",
            "2000",
            "2000",
            vec!["oaiewjf"],
        );
        assert_eq!(
            product,
            Product {
                product_id: "09320481032".to_string(),
                business_id: "102938409128305".to_string(),
                title: "aiowjefaieorf".to_string(),
                description: "oiewroaer".to_string(),
                image_url: "oijeaij".to_string(),
                price: "2000".to_string(),
                unformatted_price: "2000".to_string(),
                product_tags: vec!["oaiewjf".to_string()]
            }
        );
    }

    #[test]
    fn test_create_product() {
        let product = Product::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "aoeirojfiearg",
            "oaiejriearg",
            "http://image.com",
            "2000",
            vec!["fiejroa"],
        ).is_ok();
        assert!(product, "Product create function is working correctly");
    }

    #[test]
    fn test_new_business() {
        let business = Business::new(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "1029340918",
            vec!["01239041"],
            vec!["hamburguer"],
        );
        assert_eq!(
            business,
            Business {
                business_id: "25650673-c3e8-4cbb-a7bd-e27d268157b8".to_string(),
                user_id: "1029340918".to_string(),
                products_id: vec!["01239041".to_string()],
                business_tags: vec!["hamburguer".to_string()],
            }
        );
    }

    #[test]
    fn test_create_business() {
        let business = Business::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            vec!["25650673-c3e8-4cbb-a7bd-e27d268157b8"],
            vec!["hamburguer"],
        )
        .is_ok();
        assert!(business, "Business struct created");
    }
}
