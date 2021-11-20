use yew::{Component, ChildrenWithProps, ComponentLink, ShouldRender, Html};


pub struct Grid {
    pub link: ComponentLink<Self>,
    pub children: ChildrenWithProps<Cell>,
}

impl Component for Grid {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}

pub enum CellFlag {
    MineFlag,
    SafeFlag,
    NoFlag,
}

pub struct Cell {
    pub has_mine: bool,
    pub flag: CellFlag,
    pub clicked: bool,

}

impl Component for Cell {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}