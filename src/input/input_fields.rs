
use std::sync::Arc;
use cursive::{
    direction::Direction, 
    event::{self, Callback, Event, EventResult},
    theme::StyleType, view::CannotFocus,
    views::EditView,
    Cursive,
    Printer,
    Rect,
    Vec2,
    View
};

use super::text_area_v2::TextAreaV2;

/// `TextArea` that only accepts 0s and 1s as input.
pub struct VectorTextAreaV2(TextAreaV2);

impl VectorTextAreaV2 {
    /// Creates a new, empty TextArea.
    pub fn new() -> Self {
        Self(TextAreaV2::new())
    }

    /// Retrieves the content of the view.
    pub fn get_content(&self) -> &str {
        self.0.get_content()
    }

    /// Returns the position of the cursor in the content string.
    ///
    /// This is a byte index.
    pub fn cursor(&self) -> usize {
        self.0.cursor()
    }

    /// Moves the cursor to the given byte position.
    ///
    /// # Panics
    ///
    /// This method panics if `cursor` is not the starting byte of a character in
    /// the content string.
    pub fn set_cursor(&mut self, cursor: usize) {
        self.0.set_cursor(cursor);
    }

    /// Sets the content of the view.
    pub fn set_content<S: Into<String>>(&mut self, content: S) -> Callback {
        let content: String = content.into();

        if None == content.find(|c| c != '0' && c != '1') {
            self.0.set_content(content)
        }
        else {
            Callback::dummy()
        }
    }

    /// Sets the content of the view.
    ///
    /// Chainable variant.
    #[must_use]
    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.set_content(content);
        self
    }

    /// Disables this view.
    ///
    /// A disabled view cannot be selected.
    pub fn disable(&mut self) {
        self.0.disable()
    }

    /// Disables this view.
    ///
    /// Chainable variant.
    #[must_use]
    pub fn disabled(self) -> Self {
        Self(self.0.disabled())
    }

    /// Re-enables this view.
    pub fn enable(&mut self) {
        self.0.enable()
    }

    /// Re-enables this view.
    ///
    /// Chainable variant.
    #[must_use]
    pub fn enabled(self) -> Self {
        Self(self.0.enabled())
    }

    /// Returns `true` if this view is enabled.
    pub fn is_enabled(&self) -> bool {
        self.0.is_enabled()
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// *Warning*: this callback cannot be called recursively. If you somehow
    /// trigger this callback again in the given closure, it will be ignored.
    ///
    /// If you don't need a mutable closure but want the possibility of
    /// recursive calls, see [`set_on_edit`](#method.set_on_edit).
    pub fn set_on_edit_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit_mut(callback)
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// This callback can safely trigger itself recursively if needed
    /// (for instance if you call `on_event` on this view from the callback).
    ///
    /// If you need a mutable closure and don't care about the recursive
    /// aspect, see [`set_on_edit_mut`](#method.set_on_edit_mut).
    pub fn set_on_edit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit(callback)
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit_mut`](#method.set_on_edit_mut).
    #[must_use]
    pub fn on_edit_mut<F>(self, callback: F) -> Self
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit_mut(callback))
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit`](#method.set_on_edit).
    ///
    /// # Examples
    ///
    /// ```
    /// use cursive_core::views::{EditView, TextContent, TextView};
    /// // Keep the length of the text in a separate view.
    /// let mut content = TextContent::new("0");
    /// let text_view = TextView::new_with_content(content.clone());
    ///
    /// let on_edit = EditView::new().on_edit(move |_s, text, _cursor| {
    ///     content.set_content(format!("{}", text.len()));
    /// });
    /// ```
    #[must_use]
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit(callback))
    }
    
}

impl View for VectorTextAreaV2 {
    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.0.required_size(constraint)
    }

    fn draw(&self, printer: &Printer) {
        self.0.draw(printer)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if !self.0.is_enabled() {
            return EventResult::Ignored;
        }

        match event {
            event::Event::Char(ch) => {
                if ch == '0' || ch == '1' {
                    return self.0.on_event(event);
                }
                else {
                    return EventResult::Ignored;
                }
            },
            _ => self.0.on_event(event),
        }
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        self.0.take_focus(source)
    }

    fn layout(&mut self, size: Vec2) {
        self.0.layout(size)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.0.important_area(view_size)
    }
}

/// `EditView` that only accepts integers [0-9] as input.
#[repr(transparent)]
pub struct UnsignedEditView(EditView);

impl UnsignedEditView {
    /// Creates a new, empty vector edit view.
    #[inline]
    pub fn new() -> Self {
        Self(EditView::new())
    }

    /// Sets a maximum width for the content.
    ///
    /// Input will be rejected if it would make the content exceed this width.
    ///
    /// Giving `None` means no maximum width is applied.
    #[inline]
    pub fn set_max_content_width(&mut self, width: Option<usize>) {
        self.0.set_max_content_width(width);
    }

    /// Sets a maximum width for the content.
    ///
    /// Input will be rejected if it would make the content exceed this width.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn max_content_width(self, width: usize) -> Self {
        Self(self.0.max_content_width(width))
    }

    /// If `secret` is `true`, the content won't be displayed in clear.
    ///
    /// Only `*` will be shown.
    #[inline]
    pub fn set_secret(&mut self, secret: bool) {
        self.0.set_secret(secret)
    }

    /// Hides the content of the view.
    ///
    /// Only `*` will be shown.
    #[must_use]
    #[inline]
    pub fn secret(self) -> Self {
        Self(self.0.secret())
    }

    /// Sets the character to fill in blank space.
    ///
    /// Defaults to "_".
    #[inline]
    pub fn set_filler<S: Into<String>>(&mut self, filler: S) {
        self.0.set_filler(filler)
    }

    /// Sets the character to fill in blank space.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let edit = UnsignedEditView::new().filler(" ");
    /// ```
    #[must_use]
    #[inline]
    pub fn filler<S: Into<String>>(self, filler: S) -> Self {
        Self(self.0.filler(filler))
    }

    /// Sets the style used for this view.
    ///
    /// When the view is enabled, the style will be reversed.
    ///
    /// Defaults to `ColorStyle::Secondary`.
    #[inline]
    pub fn set_style<S: Into<StyleType>>(&mut self, style: S) {
        self.0.set_style(style)
    }

    /// Sets the style used for this view.
    ///
    /// When the view is enabled, the style will be reversed.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn style<S: Into<StyleType>>(self, style: S) -> Self {
        Self(self.0.style(style))
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// *Warning*: this callback cannot be called recursively. If you somehow
    /// trigger this callback again in the given closure, it will be ignored.
    ///
    /// If you don't need a mutable closure but want the possibility of
    /// recursive calls, see [`set_on_edit`](#method.set_on_edit).
    #[inline]
    pub fn set_on_edit_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit_mut(callback)
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// This callback can safely trigger itself recursively if needed
    /// (for instance if you call `on_event` on this view from the callback).
    ///
    /// If you need a mutable closure and don't care about the recursive
    /// aspect, see [`set_on_edit_mut`](#method.set_on_edit_mut).
    #[inline]
    pub fn set_on_edit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit(callback)
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit_mut`](#method.set_on_edit_mut).
    #[must_use]
    #[inline]
    pub fn on_edit_mut<F>(self, callback: F) -> Self
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit_mut(callback))
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit`](#method.set_on_edit).
    ///
    /// # Examples
    ///
    /// ```
    /// // Keep the length of the text in a separate view.
    /// let mut content = TextContent::new("0");
    /// let text_view = TextView::new_with_content(content.clone());
    ///
    /// let on_edit = UnsignedEditView::new().on_edit(move |_s, text, _cursor| {
    ///     content.set_content(format!("{}", text.len()));
    /// });
    /// ```
    #[must_use]
    #[inline]
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit(callback))
    }

    /// Sets a mutable callback to be called when `<Enter>` is pressed.
    ///
    /// `callback` will be given the content of the view.
    ///
    /// *Warning*: this callback cannot be called recursively. If you somehow
    /// trigger this callback again in the given closure, it will be ignored.
    ///
    /// If you don't need a mutable closure but want the possibility of
    /// recursive calls, see [`set_on_submit`](#method.set_on_submit).
    #[inline]
    pub fn set_on_submit_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Cursive, &str) + 'static + Send + Sync,
    {
        self.0.set_on_submit_mut(callback)
    }

    /// Sets a callback to be called when `<Enter>` is pressed.
    ///
    /// `callback` will be given the content of the view.
    ///
    /// This callback can safely trigger itself recursively if needed
    /// (for instance if you call `on_event` on this view from the callback).
    ///
    /// If you need a mutable closure and don't care about the recursive
    /// aspect, see [`set_on_submit_mut`](#method.set_on_submit_mut).
    #[inline]
    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, &str) + 'static + Send + Sync,
    {
        self.0.set_on_submit(callback)
    }

    /// Sets a mutable callback to be called when `<Enter>` is pressed.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn on_submit_mut<F>(self, callback: F) -> Self
    where
        F: FnMut(&mut Cursive, &str) + 'static + Send + Sync,
    {
        Self(self.0.on_submit_mut(callback))
    }

    /// Sets a callback to be called when `<Enter>` is pressed.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let edit_view = UnsignedEditView::new().on_submit(|s, text| {
    ///     s.add_layer(Dialog::info(text));
    /// });
    /// ```
    #[must_use]
    #[inline]
    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, &str) + 'static + Send + Sync,
    {
        Self(self.0.on_submit(callback))
    }

    /// Replace the entire content of the view with the given one.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn set_content<S: Into<String>>(&mut self, content: S) -> Callback {
        let content: String = content.into();

        if let Some(_) = content.find(|c: char| !c.is_ascii_digit()) {
            return Callback::dummy()
        }

        self.0.set_content(content)
    }

    /// Get the current text.
    #[allow(clippy::rc_buffer)]
    #[inline]
    pub fn get_content(&self) -> Arc<String> {
        self.0.get_content()
    }

    /// Sets the current content to the given value.
    ///
    /// Convenient chainable method.
    ///
    /// Does not run the `on_edit` callback.
    #[must_use]
    #[inline]
    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.set_content(content);
        self
    }

    /// Returns the currest cursor position.
    #[inline]
    pub fn get_cursor(&self) -> usize {
        self.0.get_cursor()
    }

    /// Sets the cursor position.
    #[inline]
    pub fn set_cursor(&mut self, cursor: usize) {
        self.0.set_cursor(cursor)
    }

    /// Insert `ch` at the current cursor position.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    #[inline]
    pub fn insert(&mut self, ch: char) -> Callback {
        if ch.is_ascii_digit() {
            self.0.insert(ch)
        }
        else {
            Callback::dummy()
        }
    }

    /// Remove the character at the current cursor position.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    #[inline]
    pub fn remove(&mut self, len: usize) -> Callback {
        self.0.remove(len)
    }
}

impl View for UnsignedEditView {
    #[inline]
    fn draw(&self, printer: &cursive::Printer) {
        self.0.draw(printer)
    }
    
    #[inline]
    fn layout(&mut self, size: cursive::Vec2) {
        self.0.layout(size);
    }

    fn on_event(&mut self, event: event::Event) -> event::EventResult {
        if !self.0.is_enabled() {
            return EventResult::Ignored;
        }

        match event {
            event::Event::Char(ch) => {
                if ch.is_ascii_digit() {
                    return EventResult::Consumed(Some(self.0.insert(ch)));
                }
                else {
                    return EventResult::Ignored;
                }
            },
            _ => self.0.on_event(event),
        }
    }

    #[inline]
    fn take_focus(&mut self, source: cursive::direction::Direction) -> Result<event::EventResult, cursive::view::CannotFocus> {
        self.0.take_focus(source)
    }

    #[inline]
    fn important_area(&self, view_size: cursive::Vec2) -> cursive::Rect {
        self.0.important_area(view_size)
    }
}


/// `EditView` that only accepts integers [0-9] and decimal separators `.` `,` as input.
#[repr(transparent)]
pub struct FloatEditView(EditView);

impl FloatEditView {
    /// Creates a new, empty vector edit view.
    #[inline]
    pub fn new() -> Self {
        Self(EditView::new())
    }

    /// Sets a maximum width for the content.
    ///
    /// Input will be rejected if it would make the content exceed this width.
    ///
    /// Giving `None` means no maximum width is applied.
    #[inline]
    pub fn set_max_content_width(&mut self, width: Option<usize>) {
        self.0.set_max_content_width(width);
    }

    /// Sets a maximum width for the content.
    ///
    /// Input will be rejected if it would make the content exceed this width.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn max_content_width(self, width: usize) -> Self {
        Self(self.0.max_content_width(width))
    }

    /// If `secret` is `true`, the content won't be displayed in clear.
    ///
    /// Only `*` will be shown.
    #[inline]
    pub fn set_secret(&mut self, secret: bool) {
        self.0.set_secret(secret)
    }

    /// Hides the content of the view.
    ///
    /// Only `*` will be shown.
    #[must_use]
    #[inline]
    pub fn secret(self) -> Self {
        Self(self.0.secret())
    }

    /// Sets the character to fill in blank space.
    ///
    /// Defaults to "_".
    #[inline]
    pub fn set_filler<S: Into<String>>(&mut self, filler: S) {
        self.0.set_filler(filler)
    }

    /// Sets the character to fill in blank space.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let edit = FloatEditView::new().filler(" ");
    /// ```
    #[must_use]
    #[inline]
    pub fn filler<S: Into<String>>(self, filler: S) -> Self {
        Self(self.0.filler(filler))
    }

    /// Sets the style used for this view.
    ///
    /// When the view is enabled, the style will be reversed.
    ///
    /// Defaults to `ColorStyle::Secondary`.
    #[inline]
    pub fn set_style<S: Into<StyleType>>(&mut self, style: S) {
        self.0.set_style(style)
    }

    /// Sets the style used for this view.
    ///
    /// When the view is enabled, the style will be reversed.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn style<S: Into<StyleType>>(self, style: S) -> Self {
        Self(self.0.style(style))
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// *Warning*: this callback cannot be called recursively. If you somehow
    /// trigger this callback again in the given closure, it will be ignored.
    ///
    /// If you don't need a mutable closure but want the possibility of
    /// recursive calls, see [`set_on_edit`](#method.set_on_edit).
    #[inline]
    pub fn set_on_edit_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit_mut(callback)
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// `callback` will be called with the view
    /// content and the current cursor position.
    ///
    /// This callback can safely trigger itself recursively if needed
    /// (for instance if you call `on_event` on this view from the callback).
    ///
    /// If you need a mutable closure and don't care about the recursive
    /// aspect, see [`set_on_edit_mut`](#method.set_on_edit_mut).
    #[inline]
    pub fn set_on_edit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        self.0.set_on_edit(callback)
    }

    /// Sets a mutable callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit_mut`](#method.set_on_edit_mut).
    #[must_use]
    #[inline]
    pub fn on_edit_mut<F>(self, callback: F) -> Self
    where
        F: FnMut(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit_mut(callback))
    }

    /// Sets a callback to be called whenever the content is modified.
    ///
    /// Chainable variant. See [`set_on_edit`](#method.set_on_edit).
    ///
    /// # Examples
    ///
    /// ```
    /// // Keep the length of the text in a separate view.
    /// let mut content = TextContent::new("0");
    /// let text_view = TextView::new_with_content(content.clone());
    ///
    /// let on_edit = FloatEditView::new().on_edit(move |_s, text, _cursor| {
    ///     content.set_content(format!("{}", text.len()));
    /// });
    /// ```
    #[must_use]
    #[inline]
    pub fn on_edit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, &str, usize) + 'static + Send + Sync,
    {
        Self(self.0.on_edit(callback))
    }

    /// Sets a mutable callback to be called when `<Enter>` is pressed.
    ///
    /// `callback` will be given the content of the view.
    ///
    /// *Warning*: this callback cannot be called recursively. If you somehow
    /// trigger this callback again in the given closure, it will be ignored.
    ///
    /// If you don't need a mutable closure but want the possibility of
    /// recursive calls, see [`set_on_submit`](#method.set_on_submit).
    #[inline]
    pub fn set_on_submit_mut<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Cursive, &str) + 'static + Send + Sync,
    {
        self.0.set_on_submit_mut(callback)
    }

    /// Sets a callback to be called when `<Enter>` is pressed.
    ///
    /// `callback` will be given the content of the view.
    ///
    /// This callback can safely trigger itself recursively if needed
    /// (for instance if you call `on_event` on this view from the callback).
    ///
    /// If you need a mutable closure and don't care about the recursive
    /// aspect, see [`set_on_submit_mut`](#method.set_on_submit_mut).
    #[inline]
    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, &str) + 'static + Send + Sync,
    {
        self.0.set_on_submit(callback)
    }

    /// Sets a mutable callback to be called when `<Enter>` is pressed.
    ///
    /// Chainable variant.
    #[must_use]
    #[inline]
    pub fn on_submit_mut<F>(self, callback: F) -> Self
    where
        F: FnMut(&mut Cursive, &str) + 'static + Send + Sync,
    {
        Self(self.0.on_submit_mut(callback))
    }

    /// Sets a callback to be called when `<Enter>` is pressed.
    ///
    /// Chainable variant.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let edit_view = FloatEditView::new().on_submit(|s, text| {
    ///     s.add_layer(Dialog::info(text));
    /// });
    /// ```
    #[must_use]
    #[inline]
    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, &str) + 'static + Send + Sync,
    {
        Self(self.0.on_submit(callback))
    }

    /// Replace the entire content of the view with the given one.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn set_content<S: Into<String>>(&mut self, content: S) -> Callback {
        let content: String = content.into();

        if let Some(_) = content.find(|c: char| {
            !c.is_ascii_digit() && c != '.' && c != ','
        }) 
        {
            return Callback::dummy()
        }

        self.0.set_content(content)
    }

    /// Get the current text.
    #[allow(clippy::rc_buffer)]
    #[inline]
    pub fn get_content(&self) -> Arc<String> {
        self.0.get_content()
    }

    /// Sets the current content to the given value.
    ///
    /// Convenient chainable method.
    ///
    /// Does not run the `on_edit` callback.
    #[must_use]
    #[inline]
    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.set_content(content);
        self
    }

    /// Returns the currest cursor position.
    #[inline]
    pub fn get_cursor(&self) -> usize {
        self.0.get_cursor()
    }

    /// Sets the cursor position.
    #[inline]
    pub fn set_cursor(&mut self, cursor: usize) {
        self.0.set_cursor(cursor)
    }

    /// Insert `ch` at the current cursor position.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    #[inline]
    pub fn insert(&mut self, ch: char) -> Callback {
        if ch.is_ascii_digit() || ch == '.' || ch == ',' {
            self.0.insert(ch)
        }
        else {
            Callback::dummy()
        }
    }

    /// Remove the character at the current cursor position.
    ///
    /// Returns a callback in response to content change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    #[inline]
    pub fn remove(&mut self, len: usize) -> Callback {
        self.0.remove(len)
    }
}

impl View for FloatEditView {
    #[inline]
    fn draw(&self, printer: &cursive::Printer) {
        self.0.draw(printer)
    }
    
    #[inline]
    fn layout(&mut self, size: cursive::Vec2) {
        self.0.layout(size);
    }

    fn on_event(&mut self, event: event::Event) -> event::EventResult {
        if !self.0.is_enabled() {
            return EventResult::Ignored;
        }

        match event {
            event::Event::Char(ch) => {
                if ch.is_ascii_digit() || ch == '.' || ch == ',' {
                    return EventResult::Consumed(Some(self.0.insert(ch)));
                }
                else {
                    return EventResult::Ignored;
                }
            },
            _ => self.0.on_event(event),
        }
    }

    #[inline]
    fn take_focus(&mut self, source: cursive::direction::Direction) -> Result<event::EventResult, cursive::view::CannotFocus> {
        self.0.take_focus(source)
    }

    #[inline]
    fn important_area(&self, view_size: cursive::Vec2) -> cursive::Rect {
        self.0.important_area(view_size)
    }
}
