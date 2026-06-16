use ghx_proc_gen::generator::builder::GeneratorBuilder;
use ghx_proc_gen::generator::model::ModelCollection;
use ghx_proc_gen::generator::rules::RulesBuilder;
use ghx_proc_gen::generator::socket::{SocketCollection, SocketsCartesian2D};
use ghx_proc_gen::ghx_grid::cartesian::coordinates::{Cartesian2D, CartesianPosition};
use ghx_proc_gen::ghx_grid::cartesian::grid::CartesianGrid;
use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct WfcNode3D {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for WfcNode3D {
    fn init(base: Base<Node3D>) -> Self {
        Self { base }
    }
}

#[godot_api]

impl WfcNode3D {
    #[func]
    fn make_road_rules(&self, size_x: i64, size_y: i64) -> Array<Gd<RoadSegment>> {
        let mut sockets = SocketCollection::new();
        let (road_end, no_road) = (sockets.create(), sockets.create());

        sockets.add_connection(no_road, vec![no_road]);
        sockets.add_connection(road_end, vec![road_end]);

        let mut models = ModelCollection::<Cartesian2D>::new();

        let straight_road_model = SocketsCartesian2D::Simple {
            x_pos: road_end,
            x_neg: road_end,
            y_pos: no_road,
            y_neg: no_road,
        }
        .to_template()
        .with_all_rotations();

        let curved_road_model = SocketsCartesian2D::Simple {
            x_pos: no_road,
            x_neg: road_end,
            y_pos: road_end,
            y_neg: no_road,
        }
        .to_template()
        .with_all_rotations();

        let sidewalk_model = SocketsCartesian2D::Simple {
            x_pos: no_road,
            x_neg: no_road,
            y_pos: no_road,
            y_neg: no_road,
        };

        let intersection_model = SocketsCartesian2D::Simple {
            x_pos: road_end,
            x_neg: road_end,
            y_pos: road_end,
            y_neg: road_end,
        };

        models.create(straight_road_model);
        models.create(curved_road_model);
        let sidewalk = models.create(sidewalk_model).clone();
        models.create(intersection_model);

        let rules = RulesBuilder::new_cartesian_2d(models, sockets)
            .build()
            .unwrap();

        let grid = CartesianGrid::new_cartesian_2d(size_x as u32, size_y as u32, false, false);
        let mut generator = GeneratorBuilder::new()
            .with_rules(rules)
            .with_grid(grid)
            // Let's ensure that we make a chessboard, with a black square bottom-left
            .with_initial_nodes(vec![(CartesianPosition::new_xy(0, 0), sidewalk)])
            .unwrap()
            .build()
            .unwrap();

        let pattern = generator.generate_grid().unwrap().1;

        let mut output = Array::new();

        for y in 0..size_x {
            for x in 0..size_y {
                let cell = pattern.get_2d(x as u32, y as u32);
                let rot = cell.rotation;
                let seg = Gd::from_object(RoadSegment::new(
                    cell.model_index as i64,
                    x as i64,
                    y as i64,
                    rot.value() as i64,
                ));
                output.push(&seg);
            }
        }

        let seg: Gd<RoadSegment> = Gd::from_object(RoadSegment {
            model_index: 0,
            x: 0,
            z: 0,
            rot: 90,
        });

        output
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct RoadSegment {
    #[var]
    model_index: i64,
    #[var]
    x: i64,
    #[var]
    z: i64,
    #[var]
    rot: i64,
}

impl RoadSegment {
    fn new(model_index: i64, x: i64, z: i64, rot: i64) -> Self {
        Self {
            model_index,
            x,
            z,
            rot,
        }
    }
}
