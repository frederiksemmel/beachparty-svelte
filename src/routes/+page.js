/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    const res = await fetch(`/api/events`);
    const item = await res.json();
    return item;
}