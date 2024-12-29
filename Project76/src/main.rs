trait Overview {
    fn overview(&self) -> String {
        String::from("This is default implementation for trait")
    }
}

struct Course {
    headline: String,
    author: String,
}
struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

impl Overview for AnotherCourse {}

fn main() {
    let course1 = Course {
        headline: String::from("Headline!"),
        author: String::from("Harsh"),
    };
    let course2 = AnotherCourse {
        headline: String::from("AnotherHeadline!"),
        author: String::from("AnotherHarsh"),
    };
    println!("{}", course1.overview());
    println!("{}", course2.overview());
}
