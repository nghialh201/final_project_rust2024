// First, we are going to introduce some units of energy. For whatever reason, we prefer BTU above
// Joules and Calories, but we want to support all 3 of these in this module. Double check the
// conversion methods, and make sure you fully understand them.

use std::marker::PhantomData;

// You may uncomment and use the following import if you need it. You may also read its
// documentation at https://doc.rust-lang.org/std/cell/struct.RefCell.html
use std::cell::RefCell;


#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Joule(pub u32);
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Calorie(pub u32);

pub type BTU = u32;

impl From<Joule> for BTU {
    fn from(j: Joule) -> Self {
        j.0 / 1055
    }
}

impl From<BTU> for Joule {
    fn from(b: BTU) -> Self {
        Self(b * 1055)
    }
}

impl From<Calorie> for BTU {
    fn from(c: Calorie) -> Self {
        c.0 / 251
    }
}

impl From<BTU> for Calorie {
    fn from(b: BTU) -> Self {
        Calorie(b * 251)
    }
}

// Now, we start defining some types of fuel.

/// A technology for storing energy for later consumption.
pub trait Fuel {
    /// The output unit of the energy density.
    ///
    /// Think about this: why did we chose this to be an associated type rather than a generic?
    type Output: Into<BTU> + From<BTU>;

    /// The amount of energy contained in a single unit of fuel.
    fn energy_density() -> Self::Output;
}

pub struct Diesel;
impl Fuel for Diesel {
    type Output = Joule;
    fn energy_density() -> Self::Output {
        // todo!("100 BTU")
        Joule::from(100)
    }
}

pub struct LithiumBattery;
impl Fuel for LithiumBattery {
    type Output = Calorie;
    fn energy_density() -> Self::Output {
        // todo!("200 BTU")
        Calorie::from(200)
    }
}

pub struct Uranium;
impl Fuel for Uranium {
    type Output = Joule;
    fn energy_density() -> Self::Output {
        // todo!("1000 BTU")
        Joule::from(1000)
    }
}

/// A container for any fuel type.
pub struct FuelContainer<F: Fuel> {
    /// The amount of fuel.
    amount: u32,
    /// NOTE: Fuel doesn't really have any methods that require `&self` on it,
    /// so any information that we can get, we can get from `F` as **TYPE**, we don't really need
    /// to store an instance of `F`, like `fuel: F` as a struct field. But to satisfy the compiler,
    /// we must use `F` somewhere.
    /// Thus, this is the perfect use case of `PhantomData`.
    _marker: PhantomData<F>,
}

impl<F: Fuel> FuelContainer<F> {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            _marker: Default::default(),
        }
    }
}

/// Something that can provide energy from a given `F` fuel type, like a power-plant.
pub trait ProvideEnergy<F: Fuel> {
    /// Consume the fuel container and return the created energy, based on the power density of the
    /// fuel and potentially other factors.
    ///
    /// Some fuel providers might have some kind of decay or inefficiency, which should be reflected
    /// here. Otherwise, [ProvideEnergy::provide_energy_with_efficiency] or
    /// [ProvideEnergy::provide_energy_ideal] might be good enough.
    ///
    /// Not all `ProvideEnergy` implementations need to have internal state. Therefore, this
    /// interface accepts `&self`, not `&mut self`. You might need to use special language features
    /// to overcome this.
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output;

    /// Convert the amount of fuel in `f` with an exact efficiency of `e`.
    ///
    /// NOTE: all efficiencies are interpreted as u8 values that can be at most 100, and represent a
    /// percent. If an efficiency above 100 is supplied, the code should treat it as 100. That is to
    /// say that the efficiency is "saturating" at 100%.
    ///
    /// This method must be provided as it will be the same in all implementations.
    fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {
        // todo!();
        let efficiency = if e > 100 {100} else {e};

        let result = F::energy_density().into() * f.amount * (efficiency as u32) /100;

        println!("Ket Qua ne: {}", result);
         
        result.into()
    }

    /// Same as [`ProvideEnergy::provide_energy_with_efficiency`], but with an efficiency of 100.
    ///
    /// This method must be provided as it will be the same in all implementations.
    fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!();
        let result = F::energy_density().into() * f.amount;
         
        result.into()
    }
}

/// A nuclear reactor that can only consume `Uranium` and provide energy with 99% efficiency.
pub struct NuclearReactor;
impl<F: Fuel> ProvideEnergy<F> for NuclearReactor {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        let result = self.provide_energy_with_efficiency( f, 99);

        result
    }
}

    /// A combustion engine that can only consume `Diesel`.
    ///
    /// The `DECAY` const must be interpreted as such: per every `DECAY` times `provide_energy` is
    /// called on an instance of this type, the efficiency should reduce by one. The initial efficiency
    /// must be configurable with a `fn new(efficiency: u8) -> Self`.
pub struct InternalCombustion<const DECAY: u32>/* Fill the fields as needed */ {efficiency: RefCell<u8>, count_decay: RefCell<u32>}

impl<const DECAY: u32> InternalCombustion<DECAY> {
    pub fn new(efficiency: u8) -> Self {

        // If an efficiency above 100 is supplied, the code should treat it as 100. That is to
        let new_efficiency = if efficiency > 100 {100} else {efficiency};
        // efficiency = efficiency;
        
        // count_decay.borrow_mut() = DECAY;
        Self {
            efficiency: RefCell::new(new_efficiency), 
            count_decay: RefCell::new(DECAY)
        }
    }

    pub fn reduce_count_decay(&self){
        
        *self.count_decay.borrow_mut() -= 1;
    }

    pub fn update_count_decay(&self, new_value: u32){
        *self.count_decay.borrow_mut() = new_value;
    }

    pub fn reduce_efficiency(&self){
        *self.efficiency.borrow_mut() -= 1;
    }
}

impl<const DECAY: u32, F: Fuel> ProvideEnergy<F> for InternalCombustion<DECAY> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        self.reduce_count_decay();

        let result = self.provide_energy_with_efficiency( f,*self.efficiency.borrow());

        //update count_decay reload, the efficiency should reduce by one
        if *self.count_decay.borrow() == 0 {
            self.update_count_decay(DECAY);
            self.reduce_efficiency();
        }

        result
    }
}

/// A hypothetical device that can, unlike the `InternalCombustion`, consume **any fuel** that's of
/// type `trait Fuel`. It can provide a fixed efficiency regardless of fuel type. As before,
/// EFFICIENCY is a u8 whose value should not exceed 100, is interpreted as a percent, and should
/// saturate at 100% when a higher value is supplied.
pub struct OmniGenerator<const EFFICIENCY: u8>;

// NOTE: implement `ProvideEnergy` for `OmniGenerator` using only one `impl` block.
impl<const EFFICIENCY: u8, F: Fuel> ProvideEnergy<F> for OmniGenerator<EFFICIENCY> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        let result = self.provide_energy_ideal(f);

        result
    }
}

/// A type that can wrap two different fuel types and mix them together.
///
/// The energy density of the new fuel type is the average of the two given, once converted to BTU.
/// The output unit should also be BTU.
///
/// This can represent a new fuel type, thus it must implement `Fuel`.
pub struct Mixed<F1: Fuel, F2: Fuel>(PhantomData<(F1, F2)>);

impl<F1: Fuel, F2: Fuel> Fuel for Mixed<F1, F2> {
    type Output = BTU;

    fn energy_density() -> Self::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        let result_1 = F1::energy_density().into();
        let result_2 = F2::energy_density().into();
        let result = (result_1 + result_2) /2;

        result
    }
}

// Now think about how you can make the mixer configurable, such that it would produce a new fuel
// with an energy density that is more influences by one type than the other.
//
// For example, you have a mixer of F1, F2, and some coefficient C1, where the energy density of the
// mixture is `F1 * C1 + F2 * (1 - C1) )` where `C1` is a ratio (which you have to represent again
// with a u8 percent).
//
// The main trick is to overcome the fact that `fn energy_density` does not take in a `self`, so the
// coefficients need to be incorporated in some other way (you've already seen examples of that in
// this file ;)).
pub struct CustomMixed<const C: u8, F1, F2>(PhantomData<(F1, F2)>);


impl<const C: u8, F1: Fuel, F2: Fuel> Fuel for CustomMixed<C, F1, F2> {
    type Output = BTU;

    fn energy_density() -> Self::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        let result_1 = F1::energy_density().into();
        let result_2 = F2::energy_density().into();
        let result = (result_1 * (C as u32) /100) + (result_2 * (100-(C as u32)) /100);

        result
    }
}

// Now, any of our existing energy providers can be used with a mix fuel.

// fn green_should_work() {
//     todo!()
// }

/// A function that returns the energy produced by the `OmniGenerator` with efficiency of 80%, when
/// the fuel type is an even a mix of `Diesel` as `LithiumBattery`;
pub fn omni_80_energy(amount: u32) -> BTU {
    // todo!();
    let enery_mix = Mixed::<Diesel,LithiumBattery>::energy_density();

    let result = enery_mix * amount * 80 / 100;

    result
}

// Finally, let's consider marker traits, and some trait bounds.

/// Some traits are just markers. They don't bring any additional functionality anything, other than
/// marking a type with some trait.
pub trait IsRenewable {}
impl IsRenewable for LithiumBattery {}

// Define the following struct such that it only provides energy if the fuel is `IsRenewable`.
//
// It has perfect efficiency.
#[derive(Debug)]
pub struct GreenEngine<F: Fuel>(pub PhantomData<F>);
impl<F: Fuel> ProvideEnergy<F> for GreenEngine<F> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        let mut result = 0;
        
        //Check value energy desity of LithiumBattery
        if <F>::energy_density().into() == <LithiumBattery>::energy_density().into(){
            result = self.provide_energy_ideal(f).into();
        }

        result.into()
    }
}



/// Define the following struct such that it only provides energy if the fuel's output type is
/// `BTU`.
///
/// It has perfect efficiency.
pub struct BritishEngine<F: Fuel>(pub PhantomData<F>);
impl<F: Fuel> ProvideEnergy<F> for BritishEngine<F> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
        // todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
        
        //Default output type is BTU
        self.provide_energy_ideal(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait ToBTU {
        fn to_btu(self) -> BTU;
    }

    impl<T: Into<BTU>> ToBTU for T {
        fn to_btu(self) -> BTU {
            self.into()
        }
    }

    #[test]
    fn nuclear() {
        let nr = NuclearReactor;
        assert_eq!(
            nr.provide_energy(FuelContainer::<Uranium>::new(10))
                .to_btu(),
            9900
        );
        assert_eq!(
            nr.provide_energy(FuelContainer::<Uranium>::new(10))
                .to_btu(),
            9900
        );
    }

    #[test]
    fn ic_1() {
        let ic = InternalCombustion::<3>::new(120);
        println!("Start 1");
        assert_eq!(
            ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
            1000
        );
        assert_eq!(
            ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
            1000
        );
        assert_eq!(
            ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
            1000
        );
        assert_eq!(
            ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
            990
        );
    }

    #[test]
    fn omni_1() {
        let og = OmniGenerator::<100>;
        assert_eq!(
            og.provide_energy(FuelContainer::<Uranium>::new(10))
                .to_btu(),
            10000
        );
        assert_eq!(
            og.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
            1000
        );
        assert_eq!(
            og.provide_energy(FuelContainer::<LithiumBattery>::new(10))
                .to_btu(),
            2000
        );
    }

    #[test]
    fn mixed_1() {
        assert_eq!(
            Mixed::<Diesel, LithiumBattery>::energy_density().to_btu(),
            150
        );
    }

    #[test]
    fn custom_mixed_1() {
        // custom with 50 is the same as Mixed.
        assert_eq!(
            CustomMixed::<50, Diesel, LithiumBattery>::energy_density().to_btu(),
            Mixed::<Diesel, LithiumBattery>::energy_density()
        );
    }

    #[test]
    fn omni_80() {
        assert_eq!(
            omni_80_energy(10),
            1200
        );
    }


    #[test]
    fn green_should_work() {
        // todo!()
        let ge = GreenEngine(PhantomData);
        assert_eq!(
            ge.provide_energy(FuelContainer::<LithiumBattery>::new(10))
                .to_btu(),
            2000
        );
        // assert_eq!(
        //     ge.provide_energy(FuelContainer::<Diesel>::new(10))
        //         .to_btu(),
        //     0
        // );
    }

    #[test]
    fn british_should_work() {
        // todo!()
        let be = BritishEngine(PhantomData);
        assert_eq!(
                be.provide_energy(FuelContainer::<LithiumBattery>::new(10))
                    .to_btu(),
                2000
            );

    }
}