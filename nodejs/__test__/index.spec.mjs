import test from 'ava'

//import { init, track, flush, close, toggleLogger } from '../index.js'
import dt from '../../output/nodejs/index.js'

test('tack simple', (t) => {
    dt.toggleLogger(true);
    dt.init("log", 200, "dt_nodejs");
    let ret = dt.track("xxx", "xx", "simple_event_from_nodejs", {
        "#app_id": "appidddd",
        "#bundle_id": "com.example",
        "custom_prop": "my value",
    });
    t.is(ret, true)
    dt.flush();
    dt.close();
})
