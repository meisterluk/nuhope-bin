fn main() {
    use new_hope::{Parameters, NewHope};

    let newhope512 = Parameters{
        n: 512,
        q: 12289,
        gamma: 10968,
        // k: 8
        // omega: 3
        // omega^-1: 8193
        // gamma^-1: 3656
        // n^-1: 12265
    };
    let _newhope1024 = Parameters{
        n: 1024,
        q: 12289,
        gamma: 7,
        // k: 8
        // omega: 49
        // omega^-1: 1254
        // gamma^-1: 8778
        // n^-1: 12277
    };
    let nh = NewHope::new(&newhope512);
    //let (pk, sk) = nh.cpa_pke_gen();
    //NewHope(_ cpa_pke_gen();

    println!("{}", _newhope1024.finite_field().element(299));

    let q = 12289;
    let n = 512;
    let one = finite_field::FiniteFieldElement::new(q, 1);
    let result = new_hope::ntt(finite_field::FiniteFieldPolynomial::new(q, vec![one, one, finite_field::FiniteFieldElement::new(q, 42)]), finite_field::FiniteFieldElement::new(q, 3), n);
}
