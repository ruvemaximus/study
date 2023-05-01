export function generateToken(length=15) {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let token = '';

    for (let i = 0; i < chars.length; i++) {
        const randomIndex = Math.floor(Math.random() * chars.length);
        token += chars[randomIndex];
    }

    return token.slice(0, length);
}