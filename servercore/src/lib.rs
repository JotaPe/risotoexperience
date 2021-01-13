use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{validate_email, validate_phone};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub user_id: String, // UUID v4
    pub email: String,
    pub phone: String,
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
        image_url: &str,
        password_hash: &str,
        confirmed: bool,
        roles: Vec<&str>,
    ) -> User {
        User {
            user_id: user_id.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            image_url: image_url.to_string(),
            password_hash: password_hash.to_string(),
            confirmed,
            roles: roles.iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(
        user_id: &str,
        email: &str,
        phone: &str,
        image_url: &str,
        password_hash: &str,
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
        Ok(User::new(
            user_id,
            email,
            phone,
            image_url,
            password_hash,
            false,
            roles,
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub product_id: String,  // UUID v4
    pub business_id: String, // UUID v4
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

impl Product {
    pub fn new(
        product_id: &str,
        business_id: &str,
        title: &str,
        description: &str,
        image_url: &str,
        tags: Vec<&str>,
    ) -> Product {
        Product {
            product_id: product_id.to_string(),
            business_id: business_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            tags: tags.iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(
        product_id: &str,
        business_id: &str,
        title: &str,
        description: &str,
        image_url: &str,
        tags: Vec<&str>,
    ) -> Result<Product, &'static str> {
        match Uuid::parse_str(product_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("UUID is not valid"),
        };
        match Uuid::parse_str(business_id) {
            Ok(uuid_id) => uuid_id,
            Err(_) => return Err("Business UUID is not valid"),
        };
        if title.len() < 5 {
            return Err("Too short title");
        } else if title.len() > 100 {
            return Err("Too big title");
        }
        if description.len() < 5 {
            return Err("Too short description");
        } else if title.len() > 500 {
            return Err("Too big description");
        }
        Ok(Product::new(
            product_id,
            business_id,
            title,
            description,
            image_url,
            tags,
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Business {
    pub user_id: String, // UUID v4
    pub products_id: Vec<String>,
}

impl Business {
    pub fn new(user_id: &str, products_id: Vec<&str>) -> Business {
        Business {
            user_id: user_id.to_string(),
            products_id: products_id.iter().map(|&val| val.to_string()).collect(),
        }
    }

    pub fn create(user_id: &str, products_id: Vec<&str>) -> Result<Business, &'static str> {
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
        Ok(Business::new(user_id, products_id))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    fn new_user() {
        let user = User::new(
            "XD",
            "email",
            "3091u2304",
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
    fn create_user() {
        let user = User::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "email@example.com",
            "+5521965237969",
            "http://image.url.com",
            "XDXD",
            vec!["admin"],
        )
        .is_ok();
        assert!(user, "User create function is working correctly");
    }

    #[test]
    fn new_product() {
        let product = Product::new(
            "09320481032",
            "102938409128305",
            "aiowjefaieorf",
            "oiewroaer",
            "oijeaij",
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
                tags: vec!["oaiewjf".to_string()]
            }
        );
    }

    #[test]
    fn create_product() {
        let product = Product::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            "aoeirojfiearg",
            "oaiejriearg",
            "aoiejroear",
            vec!["fiejroa"],
        )
        .is_ok();
        assert!(product, "Product create function is working correctly");
    }

    #[test]
    fn new_business() {
        let business = Business::new("1029340918", vec!["01239041"]);
        assert_eq!(
            business,
            Business {
                user_id: "1029340918".to_string(),
                products_id: vec!["01239041".to_string()]
            }
        );
    }

    #[test]
    fn create_business() {
        let business = Business::create(
            "25650673-c3e8-4cbb-a7bd-e27d268157b8",
            vec!["25650673-c3e8-4cbb-a7bd-e27d268157b8"],
        )
        .is_ok();
        assert!(business, "Business struct created");
    }
}
