use std::io;
#[derive(Debug)]
struct laptops{
       Company_Name:String,
       Modol_no:i32,
       launched_in:i32,
       Generation:String,
       Os_System:String,
       arch:f32,
}

impl laptops{
       fn laptops_company(self)-> String{
       let Companys = format!("Company Name {}: \n Modol no {}: \n  launched In {}: \n Generation {}: \n Os System Name {}: \n Proccesor {}:",
       self.Company_Name,self.Modol_no,self.launched_in,self.Generation,self.Os_System,self.arch);  
       Companys
       }

}


 
fn main(){
       let mut Company_Name = String::new();
       let mut Modol_no = String::new();
       let mut launched_in = String::new();
       let mut Generation = String::new();
       let mut Os_System = String::new();
       let mut arch = String::new();

       println!("Enter the Company_Name");
       io::stdin().read_line(&mut Company_Name).expect("");

       println!("Enter the Modol_Name");
       io::stdin().read_line(&mut Modol_no).expect("");
       let Modol_no : i32 = Modol_no
       .trim()
       .parse()
       .unwrap();

       println!("Enter the Launched Date");
       io::stdin().read_line(&mut launched_in).expect("");
        let launched_in : i32 = launched_in
       .trim()
       .parse()
       .unwrap();

       println!("Enter the Generation");
       io::stdin().read_line(&mut Generation).expect("");

       println!("Enter the Os_system");
       io::stdin().read_line(&mut Os_System).expect("");

       println!("Enter the arch");
       io::stdin().read_line(&mut arch).expect("");
       let arch : f32 = arch
       .trim()
       .parse()
       .unwrap();

       let Company_01 = (Company_Name.to_string(),Modol_no.to_string(),launched_in.to_string(),Generation.to_string(),Os_System.to_string(),arch.to_string());
        println!("{:?}",Company_01);
       
}