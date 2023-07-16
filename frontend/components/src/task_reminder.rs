use yew::prelude::*;

#[function_component(TaskReminder)]
pub fn task_reminder() -> Html {
    html! {
        <div class="flex-1 grow flex flex-row justify-center items-center text-white">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="text-emerald-500 w-48 h-48">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>

            <span class="grow text-9xl">{"Take out the trash"}</span>
        </div>
    }
}
