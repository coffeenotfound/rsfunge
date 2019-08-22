use std::collections::HashMap;
use crate::interpreter::{FungeAddress, FungeDimension, FungeSpaceAccessor, FungeValue, FungePageAddress};
use std::marker::PhantomData;

/// Space character (32)
const EMPTY_CELL_VALUE: i32 = 32;

pub struct FungeSpace<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	page_map: HashMap<FungeAddress, FungeSpacePage<'s, N, V, A>>,
	_unused: PhantomData<(&'s u8, N)>,
}
impl<'s, N, V, A> FungeSpace<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	pub fn new() -> Self {
		FungeSpace {
			page_map: HashMap::with_capacity(128),
			_unused: PhantomData,
		}
	}
	
	pub fn read_cell(&mut self, address: &FungeAddress) -> V {
		// Try get page at address
		let page_address = A::make_page_address(address);
		let page = self.get_page_maybe(&page_address);
		
		if let Some(page) = page {
			// Localize page address
			let local_address = A::localize_address(address);
			
			// Read value from page
			return page.read_cell(&local_address);
		}
		else {
			// Return empty value if page absent
			return V::from(EMPTY_CELL_VALUE);
		}
	}
	
	pub fn write_cell(&mut self, address: &FungeAddress, value: V) {
		// Get page (create if necessary)
		let page_address = A::make_page_address(address);
		let mut page = self.get_page_or_create(&page_address);
		
		// Localize address
		let local_address = A::localize_address(address); // TODO: This might be sped up because we already have the page address, because we already have the page address
		
		// Write cell to page
		page.write_cell(&local_address, value);
	}
	
	#[inline]
	pub fn get_page_or_create(&mut self, page_address: &FungePageAddress) -> &mut FungeSpacePage<'s, N, V, A> {
		let map = &mut self.page_map;
		
		let page = map.entry(*page_address).or_insert_with(|| {
			let page = FungeSpacePage::<N, V, A>::new();
			return page;
		});
		return page;
	}
	
	#[inline]
	pub fn get_page_maybe(&mut self, page_address: &FungePageAddress) -> Option<&mut FungeSpacePage<'s, N, V, A>> {
		let map = &mut self.page_map;
		
		let page = map.get_mut(&page_address);
		return page;
	}
}

pub struct FungeSpacePage<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	pub data: Vec<V>,
	_unused: PhantomData<(&'s u8, N, A)>,
}
impl<'s, N, V, A> FungeSpacePage<'s, N, V, A> where N: FungeDimension, V: FungeValue, A: FungeSpaceAccessor<N, V> {
	pub fn new() -> Self {
		let initial_value = V::from(EMPTY_CELL_VALUE);
		let page_cap = A::get_page_capacity();
		
		FungeSpacePage {
			data: vec![initial_value; (page_cap as usize)],
			_unused: PhantomData,
		}
	}
	
	pub fn read_cell(&self, local_address: &FungeAddress) -> V {
		let linear_index = A::address_to_page_linear_index(local_address);
		return self.data[linear_index];
	}
	
	pub fn write_cell(&mut self, local_address: &FungeAddress, value: V) {
		let linear_index = A::address_to_page_linear_index(local_address);
		self.data[linear_index] = value;
	}
}

//pub trait SpacePageAccess<N, V> where N: FungeDimension, V: FungeValue {
//	fn get_page_capacity() -> u32;
//}
//
//struct DefaultSpacePageAccess<N, V> where N: FungeDimension, V: FungeValue;
//impl<N, V> SpacePageAccess<N, V> for DefaultSpacePageAccess<typenum::U3, V> {
//	fn get_page_capacity() -> u32 {
//		3
//	}
//}

//pub trait PageCellAddressCalc<N: FungeDimension> {
//	fn calc_linear_address(address: FungeAddress) -> u32;
//	
//	fn get_page_capacity() -> u32;
//}
//
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<typenum::U1, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x()
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH
//	}
//}

//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x() + (address.y() * FUNGE_PAGE_WIDTH)
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH
//	}
//}
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x() + (address.y() * FUNGE_PAGE_WIDTH) + (address.z() * FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH)
//	}
//	
//	fn get_page_capacity() -> u32 {
//		FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH * FUNGE_PAGE_WIDTH
//	}
//}
//
//impl<N, V> PageCellAddressCalc<N> for FungeSpacePage<N, V> where N: FungeDimension, V: FungeValue {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
////		assert!(false, "PageCellAddressCalc is not impl'ed for ")
//		unimplemented!()
//	}
//	
//	fn get_page_capacity() -> u32 {
//		unimplemented!()
//	}
//}

//impl<T, E> PageCellAddressCalc for T where E: Eq<T, typenum::U1> {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//		address.x()
//	}
//}
//impl<T, E> PageCellAddressCalc for T where E: Eq<T, typenum::U2> {
//	fn calc_linear_address(address: Vector3<u32>) -> u32 {
//
//	}
//}


//pub struct FungeSpacePageKey<V: FungeValue> {
//
//}



//fn _static_assert() {
//	unsafe {
//		// Asset usize is u32
//		std::mem::transmute::<usize, [u8; 4]>(0);
//	}
//}

