// src/components/city_input.rs

use leptos::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct City {
    name: String,
    has_sellers: bool,
}

#[component]
pub fn CityInput() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (selected_city, set_selected_city) = create_signal(None::<City>);
    let (error_message, set_error_message) = create_signal(String::new());

    let cities = vec![
        City { name: "Москва".to_string(), has_sellers: true },
        City { name: "Санкт-Петербург".to_string(), has_sellers: true },
        City { name: "Новосибирск".to_string(), has_sellers: true },
        City { name: "Екатеринбург".to_string(), has_sellers: true },
        City { name: "Казань".to_string(), has_sellers: false },
    ];

    let filtered_cities = create_memo(move |_| {
        let input = input.get().to_lowercase();
        cities
            .iter()
            .filter(|city| city.name.to_lowercase().starts_with(&input))
            .cloned()
            .collect::<Vec<_>>()
    });

    let handle_select = move |city: City| {
        set_input.set(city.name.clone());
        set_selected_city.set(Some(city));
        set_error_message.set(String::new());
    };

    let handle_submit = move |_| {
        if let Some(city) = selected_city.get() {
            if !city.has_sellers {
                set_error_message.set("В указанном городе нет продавцов. Возможно, вы ошиблись при вводе города? Попробуйте ввести город заново или просто укажите другой. Если вы хотите стать первым продавцом для нового города, то вы можете сделать это при добавлении нового товара через личный кабинет".to_string());
            }
        } else {
            set_error_message.set("Пожалуйста, выберите город из списка".to_string());
        }
    };

    view! {
        <div class="max-w-md mx-auto mt-10 bg-white p-6 rounded-xl shadow-lg">
            <label for="city-input" class="block text-lg font-semibold text-gray-700 mb-2">
                "Выберите город"
            </label>
            <div class="relative">
                <input
                    id="city-input"
                    type="text"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent transition duration-300 ease-in-out"
                    placeholder="Начните вводить название города"
                    prop:value=input
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                />
                <ul class="absolute z-10 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-60 overflow-auto">
                    {move || filtered_cities.get().into_iter().map(|city| {
                        let city_clone = city.clone();
                        view! {
                            <li
                                class="cursor-pointer py-2 px-4 hover:bg-green-100 transition duration-200"
                                on:click=move |_| handle_select(city_clone.clone())
                            >
                                <span class="font-medium">{city.name}</span>
                                {if city.has_sellers {
                                    view! { <span class="ml-2 text-sm text-green-600">"(есть продавцы)"</span> }
                                } else {
                                    view! { <span class="ml-2 text-sm text-gray-400">"(нет продавцов)"</span> }
                                }}
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
            <button
                class="mt-4 w-full px-4 py-2 bg-green-600 text-white font-semibold rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 transition duration-300 ease-in-out"
                on:click=handle_submit
            >
                "Подтвердить город"
            </button>
            {move || {
                let error = error_message.get();
                if !error.is_empty() {
                    view! {
                        <p class="mt-2 text-sm text-red-600 bg-red-100 p-2 rounded">{error}</p>
                    }
                } else {
                    view! { <p></p> }
                }
            }}
        </div>
    }
}