use askama::Template;
use db_core::models::bundles::ForAdminBundleList;
use db_core::models::product::{ForAdminProductList};
use db_core::models::ProductStatus;



/// Contains template for empty containers
/// 
/// For empty list
/// 
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


/// 
#[derive(Debug)]
pub enum FormModalInput{
    Text{ field: &'static str,   label: &'static str, placeholder: &'static str },
    Number{ field: &'static str, label: &'static str, placeholder: &'static str, step: f32},

    /// 
    Option{ field: &'static str, label: &'static str, options: &'static[&'static str] },

    /// Modal can add more items into the list 
    List{ field: &'static str,  label: &'static str, list_endp: &'static str, placeholder: &'static str},
} 



// region:    --- Product Page


/// Page containing the product page 
/// 
/// See also, BundlePage, ServicePage, UserPage, ShopPage
/// 
#[derive(Debug, Template)]
#[template(path="admin/manage_product.html")]
pub struct ProductPage;


/// For product page, formating collected data to table rows
/// 
/// 
/// 
#[derive(Debug, Template)]
#[template(path="admin/product_list.html")]
pub struct ProductList{
    products: Vec<ForAdminProductList>
}

#[derive(Debug, Template)]
#[template(path="admin/bundle_list.html.j2")]
pub struct BundleList{
    bundles: Vec<ForAdminBundleList>
}

impl From<Vec<ForAdminBundleList>> for BundleList{
    fn from(value: Vec<ForAdminBundleList>) -> Self {
        Self { bundles: value }
    }
}

impl From<Vec<ForAdminProductList>> for ProductList{
    fn from(value: Vec<ForAdminProductList>) -> Self {
        Self { products: value }
    }
}

// endregion: --- Product Page





#[derive(Debug)]
pub struct ManageMetaData{
    pub load_list_endp: &'static str,
    pub submit_endp: &'static str,
    pub delete_endp: &'static str,
    pub search_endp: &'static str
}


#[derive(Debug, Template)]
#[template(path="admin/manage_pages.html.j2")]
pub struct ManageTemplate{
    pub api_data: ManageMetaData,
    pub title: &'static str,
    // col name, width(percentage)
    pub columns: &'static [(&'static str,u16)],
    // For filtering in table: text display , url
    pub filters: &'static [(&'static str, &'static str)],

    pub form_inputs: &'static [FormModalInput]
}


mod filters{
    use serde::Serialize;


    pub fn serde_to_string<T: Serialize>(
        s: T,
        _: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s = serde_json::to_string(&s).unwrap_or("[]".to_string());
        Ok(s)
    }

}

pub mod util {
    use askama::Template;
    use db_core::models::product::ProductEssential;

    #[derive(Debug, Template)]
    #[template(source = r#"
<ul class="item-search-result">
    {% for product in lists %}
    <li class="search-item" data-id="{{ product.id }}" data-name="{{ product.name }}">{{ product.name }}</li>
    {% endfor %}
</ul>
"#, ext = "html")]
    pub struct OptionProductsTemplate{
        lists: Vec<ProductEssential>
    }

    impl From<Vec<ProductEssential>> for OptionProductsTemplate{
        fn from(value: Vec<ProductEssential>) -> Self {
            Self { lists: value }
        }
    }

}
