use askama::Template;
use db_core::models::product::{ForAdminProductList, ProductStatus};


#[derive(Debug, Template)]
#[template(path="admin/empty_table.html")]
pub struct EmptyListTable{
    span_size: usize
}

impl EmptyListTable {
    pub fn new_with_size(span_size: usize) -> Self {
        Self { span_size }
    }
}

#[derive(Debug, Template)]
#[template(path="admin/manage_product.html")]
pub struct ProductPage;

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
