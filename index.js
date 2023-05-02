const { transform } = require('./binding')

function toBuffer(t = {}) {
  return Buffer.from(JSON.stringify(t))
}

module.exports.transform = function (code, config, state) {
	return transform(code, toBuffer(config), state)
}
