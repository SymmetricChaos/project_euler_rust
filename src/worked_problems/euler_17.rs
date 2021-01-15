// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

fn num_to_name(n: u32) -> String {
    let lt20 = ["","one","two","three","four","five","six","seven","eight","nine","ten","eleven","twelve",
                "thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"];
    let geq20 = ["","","twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"];
    if n == 1000 {
        return "onethousand".to_owned();
    } else if n % 100 == 0 {
        let x = (n/100) as usize;
        return [lt20[x], "hundred"].concat()
    } else if n < 100 {
        if n < 20 {
            let x = n as usize;
            return lt20[x].to_owned();
        } else {
            let x = (n/10) as usize;
            let y = (n%10) as usize;
            return [geq20[x], lt20[y]].concat();
        }
    } else {
        let x = (n/100) as usize;
        return [lt20[x], "hundredand", &num_to_name(n%100)].concat();
    }
}



pub fn euler17() -> u64 {
    let mut out = 0u64;
    for n in 1..1001 {
        out += num_to_name(n).len() as u64;
    }
    return out;
}