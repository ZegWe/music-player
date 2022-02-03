pub struct Display {
    pub from: usize,
    pub to: usize,
    pub page: (usize, usize),
}

impl Display {
    pub fn new(height: usize, content_len: usize, selected_index: usize) -> Display {
        // show items length
        let display_pages = (content_len as f32 / (height as f32)).ceil() as usize;

        let mut from = 0;
        let mut to = 0;
        let mut page = 0;
        for i in 0..display_pages {
            if selected_index < (i + 1) * height {
                from = i * height;
                to = (i * height) + height;
                page = i + 1;
                break;
            }
        }
        if to >= content_len {
            to = content_len;
        }

        Display {
            from,
            to,
            page: (page, display_pages),
        }
    }
}