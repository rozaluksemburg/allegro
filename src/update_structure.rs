use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event, EventKind};
use std::fs;
use std::sync::mpsc::channel;
use std::path::Path;

fn main() {
    // Создаем канал для получения уведомлений
    let (tx, rx) = channel();

    // Создаем наблюдателя за файлами
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    // Пути к файлам, которые нужно отслеживать
    let files_to_watch = vec![
        "src/app.rs",
        "src/components/city_input.rs",
        "src/input.css",
    ];

    // Добавляем файлы в наблюдатель
    for file in &files_to_watch {
        watcher.watch(Path::new(file), RecursiveMode::NonRecursive).unwrap();
    }

    // Основной цикл для обработки событий
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(Event { kind: EventKind::Modify(_), paths, .. }) => {
                    for path in paths {
                        println!("{:?} был изменен", path);
                    }
                    update_project_structure(&files_to_watch);
                }
                _ => (),
            },
            Err(e) => println!("Ошибка наблюдателя: {:?}", e),
        }
    }
}

// Функция для обновления структуры проекта
fn update_project_structure(files: &[&str]) {
    let project_structure_file = "struct_project.txt";
    let mut structure_content = fs::read_to_string(project_structure_file).unwrap();

    for file_path in files {
        let file_content = fs::read_to_string(file_path).unwrap();
        let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();

        // Обновление содержимого файла в структуре проекта
        let start_marker = format!("<code {}>", file_name);
        let end_marker = format!("</code {}>", file_name);

        if let Some(start_index) = structure_content.find(&start_marker).map(|index| index + start_marker.len()) {
            if let Some(end_index) = structure_content.find(&end_marker) {
                println!("Updating content for {} between {} and {}", file_name, start_index, end_index); // Debug print
                structure_content.replace_range(start_index..end_index, &format!("\n{}\n", file_content));
            } else {
                println!("End marker not found for {}", file_name); // Debug print
            }
        } else {
            println!("Start marker not found for {}", file_name); // Debug print
        }
    }

    fs::write(project_structure_file, structure_content).unwrap();
}