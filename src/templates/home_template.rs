use askama::Template; // Make sure to add askama to your dependencies in Cargo.toml

#[derive(Template)] // The derive(Template) macro generates the code needed to render your template.
#[template(path = "home.html")] // This specifies the path to the template file. 
pub struct HomeTemplate<'a> { // This struct will hold the variables that you'll use in your template.
    pub name: &'a str,
}


#[derive(Template)]
#[template(path = "todos.html")]
pub struct HomeFragment {
}