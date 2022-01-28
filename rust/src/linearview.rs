use binaryninjacore_sys::*;

use crate::binaryview::BinaryView;
use crate::disassembly::{DisassemblySettings, InstructionTextToken};
use crate::function::Function;

use crate::rc::*;

pub struct LinearViewObject {
    pub(crate) handle: *mut BNLinearViewObject,
}

impl LinearViewObject {
    pub(crate) unsafe fn from_raw(handle: *mut BNLinearViewObject) -> Ref<Self> {
        debug_assert!(!handle.is_null());

        Ref::new(Self { handle })
    }

    pub fn data_only(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewDataOnly(view.handle, settings.handle);

            Self::from_raw(handle)
        }
    }

    pub fn disassembly(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewDisassembly(view.handle, settings.handle);

            Self::from_raw(handle)
        }
    }

    pub fn lifted_il(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewLiftedIL(view.handle, settings.handle);

            Self::from_raw(handle)
        }
    }

    pub fn mlil(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewMediumLevelIL(view.handle, settings.handle);

            Self::from_raw(handle)
        }
    }

    pub fn mlil_ssa(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewMediumLevelILSSAForm(
                view.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn hlil(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewHighLevelIL(view.handle, settings.handle);

            Self::from_raw(handle)
        }
    }

    pub fn hlil_ssa(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewHighLevelILSSAForm(
                view.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn language_representation(view: &BinaryView, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewLanguageRepresentation(
                view.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_disassembly(
        function: &Function,
        settings: &DisassemblySettings,
    ) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionDisassembly(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_lifted_il(
        function: &Function,
        settings: &DisassemblySettings,
    ) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionLiftedIL(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_mlil(function: &Function, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionMediumLevelIL(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_mlil_ssa(
        function: &Function,
        settings: &DisassemblySettings,
    ) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionMediumLevelILSSAForm(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_hlil(function: &Function, settings: &DisassemblySettings) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionHighLevelIL(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_hlil_ssa(
        function: &Function,
        settings: &DisassemblySettings,
    ) -> Ref<Self> {
        unsafe {
            let handle = binaryninjacore_sys::BNCreateLinearViewSingleFunctionHighLevelILSSAForm(
                function.handle,
                settings.handle,
            );

            Self::from_raw(handle)
        }
    }

    pub fn single_function_language_representation(
        function: &Function,
        settings: &DisassemblySettings,
    ) -> Ref<Self> {
        unsafe {
            let handle =
                binaryninjacore_sys::BNCreateLinearViewSingleFunctionLanguageRepresentation(
                    function.handle,
                    settings.handle,
                );

            Self::from_raw(handle)
        }
    }
}

unsafe impl RefCountable for LinearViewObject {
    unsafe fn inc_ref(handle: &Self) -> Ref<Self> {
        Ref::new(Self {
            handle: BNNewLinearViewObjectReference(handle.handle),
        })
    }

    unsafe fn dec_ref(handle: &Self) {
        BNFreeLinearViewObject(handle.handle);
    }
}

impl ToOwned for LinearViewObject {
    type Owned = Ref<Self>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { RefCountable::inc_ref(self) }
    }
}

unsafe impl Send for LinearViewObject {}
unsafe impl Sync for LinearViewObject {}

pub struct LinearViewObjectIdentifier {
    handle: *mut BNLinearViewObjectIdentifier,
}

impl LinearViewObjectIdentifier {
    #[allow(unused)]
    pub(crate) unsafe fn from_raw(handle: *mut BNLinearViewObjectIdentifier) -> Self {
        debug_assert!(!handle.is_null());
        Self { handle }
    }
}

impl Drop for LinearViewObjectIdentifier {
    fn drop(&mut self) {
        unsafe { BNFreeLinearViewObjectIdentifier(self.handle) }
    }
}

#[derive(Eq)]
pub struct LinearViewCursor {
    pub(crate) handle: *mut binaryninjacore_sys::BNLinearViewCursor,
}

impl LinearViewCursor {
    pub(crate) unsafe fn from_raw(handle: *mut BNLinearViewCursor) -> Ref<Self> {
        debug_assert!(!handle.is_null());

        Ref::new(Self { handle })
    }

    pub fn new(root: &LinearViewObject) -> Ref<Self> {
        unsafe {
            let handle = BNCreateLinearViewCursor(root.handle);
            Self::from_raw(handle)
        }
    }

    // FIXME: can we implement clone without shadowing ToOwned?
    pub fn duplicate(&self) -> Ref<Self> {
        unsafe {
            let handle = BNDuplicateLinearViewCursor(self.handle);
            Self::from_raw(handle)
        }
    }

    pub fn before_begin(&self) -> bool {
        unsafe { BNIsLinearViewCursorBeforeBegin(self.handle) }
    }

    pub fn after_end(&self) -> bool {
        unsafe { BNIsLinearViewCursorAfterEnd(self.handle) }
    }

    pub fn valid(&self) -> bool {
        !(self.before_begin() || self.after_end())
    }

    pub fn seek_to_start(&mut self) {
        unsafe { BNSeekLinearViewCursorToBegin(self.handle) }
    }

    pub fn seek_to_end(&mut self) {
        unsafe { BNSeekLinearViewCursorToEnd(self.handle) }
    }

    pub fn seek_to_address(&mut self, address: u64) {
        unsafe { BNSeekLinearViewCursorToAddress(self.handle, address) }
    }

    pub fn ordering_index_total(&self) -> u64 {
        unsafe { BNGetLinearViewCursorOrderingIndexTotal(self.handle) }
    }

    pub fn seek_to_ordering_index(&mut self, idx: u64) {
        unsafe { BNSeekLinearViewCursorToAddress(self.handle, idx) }
    }

    pub fn previous(&mut self) -> bool {
        unsafe { BNLinearViewCursorPrevious(self.handle) }
    }

    pub fn next(&mut self) -> bool {
        unsafe { BNLinearViewCursorNext(self.handle) }
    }

    pub fn lines(&self) -> LinearViewLinesIterator {
        let mut count: usize = 0;

        let lines = unsafe { BNGetLinearViewCursorLines(self.handle, &mut count) };
        LinearViewLinesIterator::new(count, lines)
    }
}

impl PartialEq for LinearViewCursor {
    fn eq(&self, other: &Self) -> bool {
        unsafe { BNCompareLinearViewCursors(self.handle, other.handle) == 0 }
    }
}

impl PartialOrd for LinearViewCursor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match unsafe { BNCompareLinearViewCursors(self.handle, other.handle) } {
            i if i < 0 => Some(std::cmp::Ordering::Less),
            i if i > 0 => Some(std::cmp::Ordering::Greater),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for LinearViewCursor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match unsafe { BNCompareLinearViewCursors(self.handle, other.handle) } {
            i if i < 0 => std::cmp::Ordering::Less,
            i if i > 0 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

unsafe impl RefCountable for LinearViewCursor {
    unsafe fn inc_ref(handle: &Self) -> Ref<Self> {
        Ref::new(Self {
            handle: BNNewLinearViewCursorReference(handle.handle),
        })
    }

    unsafe fn dec_ref(handle: &Self) {
        BNFreeLinearViewCursor(handle.handle);
    }
}

impl ToOwned for LinearViewCursor {
    type Owned = Ref<Self>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { RefCountable::inc_ref(self) }
    }
}

unsafe impl Send for LinearViewCursor {}
unsafe impl Sync for LinearViewCursor {}

pub struct LinearViewLinesIterator {
    count: usize,
    lines: *mut BNLinearDisassemblyLine,
    offset: usize,
}

impl LinearViewLinesIterator {
    pub(crate) fn new(count: usize, lines: *mut BNLinearDisassemblyLine) -> Self {
        Self {
            count,
            lines,
            offset: 0,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            count: 0,
            lines: std::ptr::null_mut(),
            offset: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl Iterator for LinearViewLinesIterator {
    type Item = LinearDisassemblyLine;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.count {
            return None;
        }
        let line = unsafe { LinearDisassemblyLine::from_raw(self.lines.offset(self.offset as _)) };

        self.offset += 1;
        Some(line)
    }
}


// FIXME: we get a segmentation fault if this is there. However,
// I don't understand enough of the API to see where this should get freed otherwise.
/*
impl Drop for LinearViewLinesIterator {
    fn drop(&mut self) {
         unsafe {
             BNFreeLinearDisassemblyLines(self.lines, self.count);
        }
    }
}
*/

pub type LinearDisassemblyLineType = BNLinearDisassemblyLineType;

pub struct LinearDisassemblyLine {
    t: LinearDisassemblyLineType,
    function: Ref<Function>,
    contents: BNDisassemblyTextLine,
}

impl LinearDisassemblyLine {
    pub(crate) unsafe fn from_raw(raw: *mut BNLinearDisassemblyLine) -> Self {
        let line = *raw;
        let linetype = line.type_;
        let function = Function::from_raw(line.function);
        let contents = line.contents;
        Self {
            t: linetype,
            function,
            contents,
        }
    }

    pub fn addr(&self) -> u64 {
        self.contents.addr
    }

    pub fn instr_idx(&self) -> usize {
        self.contents.instrIndex
    }

    pub fn count(&self) -> usize {
        self.contents.count
    }

    pub fn tag_count(&self) -> usize {
        self.contents.tagCount
    }

    pub fn tokens(&self) -> LinearDisassemblyLineTokenIterator {
        LinearDisassemblyLineTokenIterator::new(self.contents.count, self.contents.tokens)
    }

    pub fn function(&self) -> Ref<Function> {
        self.function.clone()
    }

    pub fn line_type(&self) -> LinearDisassemblyLineType {
        self.t
    }
}

impl std::fmt::Display for LinearDisassemblyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens() {
            write!(f, "{}", token.text())?;
        }

        Ok(())
    }
}


pub struct LinearDisassemblyLineTokenIterator {
    count: usize,
    tokens: *mut BNInstructionTextToken,
    offset: usize,
}

impl LinearDisassemblyLineTokenIterator {
    pub(crate) fn new(count: usize, tokens: *mut BNInstructionTextToken) -> Self {
        Self {
            count,
            tokens,
            offset: 0,
        }
    }

    #[allow(unused)]
    pub(crate) fn empty() -> Self {
        Self {
            count: 0,
            tokens: std::ptr::null_mut(),
            offset: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl Iterator for LinearDisassemblyLineTokenIterator {
    type Item = InstructionTextToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.count {
            return None;
        }

        let token = unsafe { *self.tokens.offset(self.offset as _) };

        self.offset = self.offset + 1;
        Some(InstructionTextToken(token))
    }
}
