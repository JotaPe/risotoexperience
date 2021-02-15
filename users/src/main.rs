use servercore::Business;
use servercore::User;
use tonic::{transport::Server, Code, Request, Response, Status};
use users::user_service_server::{UserService, UserServiceServer};
use users::{BusinessData, BusinessResponseData, UserData, UserResponseData};
use uuid::Uuid;
pub mod users {
    tonic::include_proto!("users");
}

#[derive(Default)]
struct UserGRPCData {}

#[tonic::async_trait]
impl UserService for UserGRPCData {
    async fn create_user(
        &self,
        request: Request<UserData>,
    ) -> Result<Response<UserResponseData>, Status> {
        let email = &request.get_ref().email;
        let password = &request.get_ref().password;
        let phone = &request.get_ref().phone;
        let address = &request.get_ref().address;
        let image_url = &request.get_ref().image_url;
        let roles = vec!["user"];
        let user = match User::create(
            &Uuid::new_v4().to_string(),
            email,
            phone,
            password,
            image_url,
            address,
            false,
            roles,
        ) {
            Ok(user) => user,
            Err(e) => return Err(Status::new(Code::InvalidArgument, e)),
        };
        Ok(Response::new(UserResponseData {
            user_id: user.user_id,
            email: user.email,
            phone: user.phone,
            address: user.address,
            image_url: user.image_url,
            roles: vec!["user".to_string()],
        }))
    }

    async fn create_business(
        &self,
        request: Request<BusinessData>,
    ) -> Result<Response<BusinessResponseData>, Status> {
        let user_id = &Uuid::new_v4().to_string();
        let business_id = &Uuid::new_v4().to_string();
        let email = &request.get_ref().email;
        let phone = &request.get_ref().phone;
        let password = &request.get_ref().password;
        let address = &request.get_ref().address;
        let image_url = &request.get_ref().image_url;
        let roles = vec!["business"];
        let user = match User::create(
            user_id,
            email,
            phone,
            password,
            image_url,
            address,
            false,
            roles,
        ) {
            Ok(user) => user,
            Err(e) => return Err(Status::new(Code::InvalidArgument, e)),
        };
        let business = match Business::create(
            business_id,
            user_id,
            Vec::default(),
            Vec::default(),
        ) {
            Ok(business) => business,
            Err(e) => return Err(Status::new(Code::InvalidArgument, e)),
        };
        Ok(Response::new(BusinessResponseData {
            user_id: business.user_id,
            business_id: business.business_id,
            email: user.email,
            phone: user.phone,
            address: user.address,
            image_url: user.image_url,
            roles: user.roles,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user_service = UserGRPCData::default();
    println!("User service listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
