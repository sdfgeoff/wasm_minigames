
use super::transform::Transform2d;
use super::trail::{PathPoint, Trail};
use super::ship::Ship;

pub struct Logo {
    pub trails: Vec<Trail>,
    pub ships: Vec<Ship>
}

impl Logo {
    pub fn new() -> Self {
    
        let mut out = Self {
            trails: vec![],
            ships: vec![]
        };
        
        
        let mut trail = Trail::new(
            (0.699999988079071, 0.0, 1.0, 1.0),
            1.0,
            1.0,
        );
        trail.path.push_front(PathPoint {
            position: (-2.191507339477539, -0.6047587394714355),
            tangent: (-3.393470287322998, -0.7639043927192688),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.9643271565437317, 0.12513494491577148),
            tangent: (0.225080206990242, -0.3531510829925537),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.6066954731941223, 0.765639066696167),
            tangent: (-0.9081510305404663, -0.5753495693206787),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        out.trails.push(trail);

        let mut trail = Trail::new(
            (0.0, 0.699999988079071, 1.0, 1.0),
            1.0,
            1.0,
        );
        trail.path.push_front(PathPoint {
            position: (-1.6977804899215698, -0.6295332312583923),
            tangent: (-1.0468578338623047, -0.320684015750885),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.3643947243690491, -0.4821237325668335),
            tangent: (-0.7735267877578735, 0.31088986992836),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.531710147857666, -0.5883793830871582),
            tangent: (-0.6054399013519287, -0.2838524281978607),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.9571377038955688, -0.1491490602493286),
            tangent: (-0.15130670368671417, -0.3664151430130005),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (1.0040268898010254, 0.16260482370853424),
            tangent: (0.1190018355846405, -0.0012154459254816175),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.9755854606628418, -0.02517738938331604),
            tangent: (-0.07657913118600845, 0.24377663433551788),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (1.26850426197052, 0.03430747985839844),
            tangent: (-0.09461011737585068, -0.39859211444854736),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.9430969953536987, 0.19939441978931427),
            tangent: (0.29438111186027527, 2.5704501638301736e-08),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.5389125347137451, -0.01779770851135254),
            tangent: (0.005455386359244585, 0.46563398838043213),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.8083686828613281, -0.06379297375679016),
            tangent: (-0.1430804431438446, -0.31913429498672485),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.5108306407928467, 0.19939446449279785),
            tangent: (0.5977222323417664, 5.140900327660347e-08),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.13235235214233398, -0.028106600046157837),
            tangent: (0.0056792558170855045, 0.4534308612346649),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.40655994415283203, -0.05580809712409973),
            tangent: (-0.15973614156246185, -0.3390654921531677),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (0.19818681478500366, 0.19542419910430908),
            tangent: (0.23441903293132782, 0.006241601426154375),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.05737864971160889, -0.04829132556915283),
            tangent: (0.14270180463790894, 0.43470096588134766),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.3142252564430237, -0.148745596408844),
            tangent: (0.17283405363559723, -0.2140694260597229),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.31960970163345337, -0.019110023975372314),
            tangent: (-0.06848830729722977, 0.007318723015487194),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.34651315212249756, -0.152736634016037),
            tangent: (0.16624715924263, 0.21063756942749023),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.5600208044052124, -0.1896149218082428),
            tangent: (0.16500118374824524, -0.10564947873353958),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        trail.path.push_front(PathPoint {
            position: (-0.5284655690193176, 0.2345539629459381),
            tangent: (-0.3030374348163605, -0.4758681356906891),
            intensity: 1.0,
            width: 0.019999999552965164,
            brightness: 100.0,
        });
        out.trails.push(trail);

        
        
        let mut ship = Ship::new(
            (0.699999988079071, 0.0, 1.0, 1.0),
        );
        ship.position = Transform2d {
    x: -0.547735333442688,
    y: 0.7970374226570129,
    rot: -1.0471975803375244,
    scale: 0.13472819328308105
};
        out.ships.push(ship);

        let mut ship = Ship::new(
            (0.0, 0.699999988079071, 1.0, 1.0),
        );
        ship.position = Transform2d {
    x: -0.494617760181427,
    y: 0.2966386675834656,
    rot: -0.5231648683547974,
    scale: 0.13472819328308105
};
        out.ships.push(ship);

        
        out
    }
}

