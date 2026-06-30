fn main() {
    let now = chrono::Local::now();

    let th_months = ["ม.ค.", "ก.พ.", "มี.ค.", "เม.ย.", "พ.ค.", "มิ.ย.",
                     "ก.ค.", "ส.ค.", "ก.ย.", "ต.ค.", "พ.ย.", "ธ.ค."];
    let month_th = th_months[(now.format("%m").to_string().parse::<usize>().unwrap()) - 1];
    let year_be = now.format("%Y").to_string().parse::<u32>().unwrap() + 543;

    println!("cargo:rustc-env=BUILD_DATE_TH={} {} {}", now.format("%d"), month_th, year_be);
    println!("cargo:rustc-env=BUILD_DATE_EN={}", now.format("%d %b %Y"));
}
