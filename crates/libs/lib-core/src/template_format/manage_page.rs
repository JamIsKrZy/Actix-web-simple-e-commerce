use askama::Template;
use db_core::models::product::ForAdminProductList;



#[derive(Debug, Template)]
#[template(path="admin/manage_product.html")]
pub struct ControlPage;

#[derive(Debug, Template)]
#[template(path="admin/product_list.html")]
pub struct ProductList{
    products: Vec<ForAdminProductList>
}

impl From<Vec<ForAdminProductList>> for ProductList{
    fn from(value: Vec<ForAdminProductList>) -> Self {
        Self { products: value }
    }
}
