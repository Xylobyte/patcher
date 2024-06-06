export const convertISOToLocalDate = (isoDateString: string) => {
    const date = new Date(isoDateString);
    const options = {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: 'numeric',
        minute: 'numeric',
        second: 'numeric'
    };
    return date.toLocaleDateString(undefined, options);
}