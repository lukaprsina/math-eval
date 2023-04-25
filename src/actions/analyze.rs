use crate::ast::{
    context::ContextAnalysis, element::ElementCache, Element, Node, NodeOrExpression,
};

impl Element {
    pub fn analyze(&mut self, mut analysis: Option<&mut ContextAnalysis>) {
        self.apply_to_every_element_mut(
            &mut |elem| {
                analyze_element(elem, &mut analysis);
            },
            false,
            None,
        );
    }
}

fn analyze_element(element: &mut Element, analysis: &mut Option<&mut ContextAnalysis>) {
    let mut nested_elem_cache = ElementCache::new();

    if element.cache.is_none() {
        element.cache = Some(ElementCache::new());
    }

    let cache = element.cache.as_mut().expect("No element cache");

    if let NodeOrExpression::Node(node) = &mut element.node_or_expression {
        match node {
            Node::Function { name, arguments: _ } => {
                if let Some(analysis) = analysis {
                    analysis.functions.insert(name.clone(), None);
                }
                cache.functions.insert(name.clone());
            }
            Node::Variable(name) => {
                if let Some(analysis) = analysis {
                    analysis.variables.insert(name.clone(), None);
                }
                cache.variables.insert(name.clone());
            }
            _ => (),
        }
    }

    element.apply_to_every_element_mut(
        &mut |elem_inner| {
            // println!("{:#?}", elem_inner);
            if let Some(cache) = &mut elem_inner.cache {
                // println!("{:#?}", cache);
                nested_elem_cache.functions.extend(cache.functions.clone());
                nested_elem_cache.variables.extend(cache.variables.clone());
            }
        },
        false,
        Some(1),
    );

    if let Some(cache) = element.cache.as_mut() {
        cache.functions.extend(nested_elem_cache.functions.clone());
        cache.variables.extend(nested_elem_cache.variables.clone());
    }
}
