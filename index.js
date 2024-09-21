const { transform } = require('./binding')

module.exports.transform = function (code, config, state) {
	return transform(code, config, state)
}
