extern crate winreg;

use winreg::enums::*;
use winreg::RegKey;
use rand::Rng;

/*key.delete_value("SomeCachedValue")
        .expect("Не удалось удалить значение из реестра");

    println!("Кэш в реестре успешно сброшен!");*/

fn main() {

    println!("Запуск HWID\nУдоставерьтесь, что Программа имеет права администратора");

    // ключи
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let hklm1 = RegKey::predef(HKEY_LOCAL_MACHINE); 

    let subkey = hklm1.open_subkey_with_flags(r#"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001"#, KEY_READ).unwrap();

    // Читаем значение по ключу
    let value: String = subkey.get_value("HwProfileGuid").unwrap();
    let value1: String = subkey.get_value("HwProfileGuid").unwrap();

    println!("Значение ключа HwProfileGuid Curent user: {}", value);
    println!("Значение ключа HwProfileGuid Local machine: {}", value1);

    println!("\nВнимание!\nHWid будет заменён на случайные.\n\n");
    match hklm1.open_subkey_with_flags(
        r#"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001"#,
        KEY_WRITE,
    ) {
        Ok(subkey) => {
            let mut rng = rand::thread_rng();
            let random_number: i32 = rng.gen_range(10000..90001);
            let generate = format!("{{furry-lover-{}-{}}}", random_number, random_number*9);
            match subkey.set_value("HwProfileGuid", &generate) {
                Ok(_) => println!("Value updated successfully"),
                Err(err) => eprintln!("Error updating value: {:?}", err),
            }
        }
        Err(err) => eprintln!("Error opening subkey: {:?}", err),
    }

    match hklm.open_subkey_with_flags(
        r#"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001"#,
        KEY_WRITE,
    ) {
        Ok(subkey) => {
            let mut rng1 = rand::thread_rng();
            let random_number1: i32 = rng1.gen_range(1000..9000)*9;
            let generate1 = format!("{{furry-lover-{}-{}}}", random_number1, random_number1*9);
            match subkey.set_value("HwProfileGuid", &generate1) {
                Ok(_) => println!("Value updated successfully"),
                Err(err) => eprintln!("Error updating value: {:?}", err),
            }
        }
        Err(err) => eprintln!("Error opening subkey: {:?}", err),
    }

    let value: String = subkey.get_value("HwProfileGuid").unwrap();
    let value1: String = subkey.get_value("HwProfileGuid").unwrap();

    println!("\n\nЗначение ключа HwProfileGuid Curent user: {}", value);
    println!("Значение ключа HwProfileGuid Local machine: {}", value1);
    
    loop {
        
    }
}

