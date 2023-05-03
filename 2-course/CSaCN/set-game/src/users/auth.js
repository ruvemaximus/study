import { Users } from "./model.js";


export const login = async function (req, res, next) {
    const bearerToken = req.headers.authorization;

    if (bearerToken === undefined) {
        return res.status(401).json({detail: 'Expected token'});
    }

    const [prefix, token] = bearerToken.split(' ');

    if (prefix !== 'Bearer') {
        return res.status(401).json({detail: 'Wrong bearer token format'});
    }

    const user = await Users.getByToken(token);

    if (user === undefined) {
        return res.status(401).json({detail: 'Wrong token'});
    }

    req.user = user;
    next();
}