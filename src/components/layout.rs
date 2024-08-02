// src/components/layout.rs

use leptos::*;
use crate::components::city_input::CityInput;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-gray-100">
            <nav class="w-1/3 bg-gradient-to-b from-green-600 to-green-800 text-white p-6 shadow-lg">
                <ul class="space-y-6">
                    <li class="hover:bg-green-700 p-3 rounded-lg transition duration-300 ease-in-out cursor-pointer">"Магазин"</li>
                    <li class="hover:bg-green-700 p-3 rounded-lg transition duration-300 ease-in-out cursor-pointer">"Личный кабинет"</li>
                    <li class="hover:bg-green-700 p-3 rounded-lg transition duration-300 ease-in-out cursor-pointer">"Сообщения"</li>
                    <li class="hover:bg-green-700 p-3 rounded-lg transition duration-300 ease-in-out cursor-pointer">"ИИ-ассистент"</li>
                </ul>
            </nav>
            <main class="w-2/3 p-8 bg-white shadow-xl">
                <CityInput />
            </main>
        </div>
    }
}

