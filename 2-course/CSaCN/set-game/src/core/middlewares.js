import { ErrorResponse } from "./responses.js";


export const expectedBody = function(expectedParams) {
    return function (req, res, next) {
        const expectedFields = expectedParams.filter(field => req.body[field] === undefined);
        console.log(expectedFields);
        if (expectedFields.length > 0) {
            return res.status(422).json(new ErrorResponse(`Fields expected: ${expectedFields.join(', ')}`));
        }
        next();
    }
}