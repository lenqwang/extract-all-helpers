use std::{collections::HashSet};

use handlebars::{
  template::{HelperTemplate, Parameter, TemplateElement},
  Template, TemplateError,
};

use crate::utils::read_file_content;

pub(crate) fn get_helpers(file_path: &str) -> HashSet<String> {
  // let start = Instant::now();
  let content = read_file_content(file_path).unwrap_or(String::default());
  let helpers = compile_hbs(content)
    .unwrap_or(HashSet::default())
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<HashSet<_>>();

  // println!("helpers: {:?}, size: {}", helpers, helpers.len());
  // println!("Time elapsed: {:?}", start.elapsed());
  
  helpers
}

fn compile_hbs(content: String) -> Result<HashSet<String>, TemplateError> {
  let mut helpers = HashSet::new();
  let template = Template::compile(&content)?;

  // println!("{:?}", template);

  process_templates(template.elements, &mut helpers);

  Ok(helpers)
}

fn process_templates(elements: Vec<TemplateElement>, helpers: &mut HashSet<String>) {
  for elem in elements.into_iter() {
    if let Some(helper_name) = extract_helper_name(elem, helpers) {
      helpers.insert(helper_name);
    }
  }
}

fn extract_helper_name(elem: TemplateElement, helpers: &mut HashSet<String>) -> Option<String> {
  match elem {
    TemplateElement::Expression(expr)
    | TemplateElement::HelperBlock(expr)
    | TemplateElement::HtmlExpression(expr) => match &*expr {
      HelperTemplate {
        name,
        params,
        hash,
        block_param: _,
        template,
        inverse,
        block: _,
      } => {
        let helper_name = match name {
          Parameter::Name(hn) => hn.to_string(),
          _ => String::from(""),
        };

        // handle params
        for param in params {
          process_parameter(param, helpers);
        }
        // handle hash
        for (_, param) in hash {
          process_parameter(param, helpers)
        }
        // handle template
        process_template_elements(template.to_owned(), helpers);

        // handle inverse
        process_template_elements(inverse.to_owned(), helpers);

        Some(helper_name)
      }
    },
    _ => None,
  }
}

fn process_parameter(param: &Parameter, helpers: &mut HashSet<String>) {
  match param {
    Parameter::Subexpression(sub_elem) => {
      if let Some(helper_name) =
        extract_helper_name(sub_elem.element.as_ref().clone(), helpers)
      {
        helpers.insert(helper_name);
      }
    }
    _ => (),
  }
}

fn process_template_elements(template: Option<Template>, helpers: &mut HashSet<String>) {
  let template_elements = if let Some(tpl) = template {
    get_template_elements(tpl.to_owned())
  } else {
    get_template_elements(Template::default())
  };
  process_templates(template_elements, helpers);
}

fn get_template_elements(template: Template) -> Vec<TemplateElement> {
  template.elements
}
