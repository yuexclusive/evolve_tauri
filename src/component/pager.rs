use super::pager_item::PagerItem;
use serde::Deserialize;
use yew::{prelude::*, Properties};

const SPAN: usize = 8;

pub const DEFAULT_PAGE_SIZE: usize = 18;

#[derive(Deserialize, PartialEq, Clone, Copy)]
pub struct Page {
    pub total: usize,
    pub index: usize,
    pub size: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            total: 0,
            index: 1,
            size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl Page {
    pub fn page_total(&self) -> usize {
        (self.total.max(1) - 1) / self.size + 1
    }

    pub fn page_start(&self) -> usize {
        ((self.index - 1) / SPAN) * SPAN + 1
    }

    pub fn page_end(&self) -> usize {
        (self.page_start() + SPAN - 1).min(self.page_total())
    }

    pub fn change_total(&mut self, total: usize) -> &mut Self {
        self.total = total;
        self
    }

    pub fn to_start(&mut self) -> &mut Self {
        self.index = 1;
        self
    }

    pub fn to_end(&mut self) -> &mut Self {
        self.index = self.page_total();
        self
    }

    pub fn to(&mut self, index: usize) -> &mut Self {
        self.index = index;
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.index += 1;
        self
    }

    pub fn pre(&mut self) -> &mut Self {
        self.index -= 1;
        self
    }

    pub fn is_start(&self) -> bool {
        self.index == 1
    }

    pub fn is_end(&self) -> bool {
        self.index == self.page_total()
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.index == index
    }

    pub fn change_size(&mut self, size: usize) -> &mut Self {
        self.size = size;
        self
    }
}

#[derive(PartialEq, Properties)]
pub struct PagerProps {
    pub index: usize,
    pub total: usize,
    pub onpagechanged: Callback<Page>,
}

#[function_component(Pager)]
pub fn pager(props: &PagerProps) -> Html {
    let page = use_mut_ref(|| Page::default());
    {
        let page = page.clone();
        use_memo(
            move |&total| {
                page.borrow_mut().change_total(total);
            },
            props.total,
        );
    }
    {
        let page = page.clone();
        use_memo(
            move |&index| {
                page.borrow_mut().to(index);
            },
            props.index,
        );
    }
    let onpagechanged = props.onpagechanged.clone();

    let size_change = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |e: web_sys::Event| {
            let el: web_sys::HtmlInputElement = e.target_unchecked_into();
            let size = el.value().parse::<usize>().unwrap();
            onpagechanged.emit(page.borrow_mut().change_size(size).clone());
        })
    };

    let to_start = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |_| {
            onpagechanged.emit(page.borrow_mut().to_start().clone());
        })
    };

    let to_end = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |_| {
            onpagechanged.emit(page.borrow_mut().to_end().clone());
        })
    };

    let to_next = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |_| {
            onpagechanged.emit(page.borrow_mut().next().clone());
        })
    };

    let to_pre = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |_| {
            onpagechanged.emit(page.borrow_mut().pre().clone());
        })
    };

    let to_index = {
        let page = page.clone();
        let onpagechanged = onpagechanged.clone();
        Callback::from(move |index: usize| {
            onpagechanged.emit(page.borrow_mut().to(index).clone());
        })
    };

    let pre_class = if page.borrow().is_start() {
        "pagination-link is-disabled"
    } else {
        "pagination-link"
    };

    let next_class = if page.borrow().is_end() {
        "pagination-link is-disabled"
    } else {
        "pagination-link"
    };

    let pre_style = if page.borrow().is_start() {
        "pointer-events: none;"
    } else {
        ""
    };
    let next_style = if page.borrow().is_end() {
        "pointer-events: none;"
    } else {
        ""
    };

    html! {
        <nav class="pagination is-small is-rounded is-right ">
        <ul class="pagination-list">
            <li>
                <div class="select is-small is-rounded">
                    <select onchange={size_change}>
                        {
                            (0..=3).map(|i|{
                                let size = DEFAULT_PAGE_SIZE<<i;
                                html!{
                                    <option selected={page.borrow().size==size} value={size.to_string()}>{format!("size: {}",size)}</option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                </div>
            </li>
            <li><a href={format!("javascript:void(0)")} class={pre_class} style={pre_style} onclick = {to_start}>{"<<"}</a></li>
            <li><a href={format!("javascript:void(0)")} class={pre_class} style={pre_style} onclick = {to_pre}>{"<"}</a></li>
            {
                (page.borrow().page_start()..= page.borrow().page_end()).map(|x|{
                    html!{
                        <PagerItem index={x} active={page.borrow().is_active(x)} onclick={to_index.clone()}/>
                    }
                }).collect::<Html>()
            }
            // <li><a class="pagination-link is-current" aria-label="Page 46" aria-current="page">{46}</a></li
            <li><a href={format!("javascript:void(0)")} class={next_class} style={next_style} onclick = {to_next}>{">"}</a></li>
            <li><a href={format!("javascript:void(0)")} class={next_class} style={next_style} onclick = {to_end}>{">>"}</a></li>
            <li>{"-- total pages:"}<b>{page.borrow().page_total()}</b>{" total records: "}<b>{page.borrow().total}</b>{" --"}</li>
        </ul>
        </nav>
    }
}
