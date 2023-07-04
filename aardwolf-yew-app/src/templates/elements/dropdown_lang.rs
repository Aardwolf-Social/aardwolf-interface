use yew::prelude::*;

#[function_component(DropdownLang)]
pub fn dropdown_lang() -> Html {

    html!{
        <>
            <div class="field">
                <p class="control is-expanded has-icons-left has-icons-right">
                    <div class="select is-narrow">
                        <select>
                        <option value="english">{"English"}</option>
                        <option value="awoo">{"Awoo"}</option>
                        <option value="japanese">{"Japanese"}</option>
                        <option value="parcel_tongue">{"Parcel Tongue"}</option>
                        <option value="polish">{"Polish"}</option>
                        <option value="redneck">{"Redneck"}</option>
                        <option value="urdu">{"Urdu"}</option>
                        </select>
                    </div>
                </p>
        </div>
        </>
    }
}