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

    pub fn lines(&self) -> Array<LinearDisassemblyLine> {
        let mut count: usize = 0;
        unsafe {
            let handles =  BNGetLinearViewCursorLines(self.handle, &mut count);
            Array::new(handles, count, ())
        }
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


pub type LinearDisassemblyLineType = BNLinearDisassemblyLineType;

pub struct LinearDisassemblyLine {
    t: LinearDisassemblyLineType,
    function: Ref<Function>,
    contents: BNDisassemblyTextLine,
}

impl LinearDisassemblyLine {
    pub(crate) unsafe fn from_raw(raw: &BNLinearDisassemblyLine) -> Self {
        let linetype = raw.type_;
        let function = Function::from_raw(raw.function);
        let contents = raw.contents;
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

    pub fn tokens(&self) -> Array<InstructionTextToken> {
        unsafe { Array::new(self.contents.tokens, self.contents.count, ()) }
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
        for token in self.tokens().iter() {
            write!(f, "{}", token.text())?;
        }

        Ok(())
    }
}

unsafe impl CoreOwnedArrayProvider for LinearDisassemblyLine {
    type Raw = BNLinearDisassemblyLine;
    type Context = ();

    unsafe fn free(raw: *mut BNLinearDisassemblyLine, count: usize, _context: &()) {
        // FIXME: If freeing, we abort with:
        // <program>(<pid>,0x108d9ce00) malloc: *** error for object 0x7fd8a66d4c90: pointer being freed was not allocated
        // <program>(<pid>,0x108d9ce00) malloc: *** set a breakpoint in malloc_error_break to debug
        // [1]    <pid> abort      cargo run /bin/cat

        // BNFreeLinearDisassemblyLines(raw, count);
    }
}

unsafe impl<'a> CoreOwnedArrayWrapper<'a> for LinearDisassemblyLine {
    type Wrapped = Guard<'a, LinearDisassemblyLine>;

    unsafe fn wrap_raw(raw: &'a Self::Raw, _context: &'a Self::Context) -> Self::Wrapped {
        Guard::new(LinearDisassemblyLine::from_raw(raw), _context)
    }
}
