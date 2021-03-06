use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
struct Line(usize, usize, usize, usize);

fn day5(lines: &[Line]) -> usize {
    const ROWS: usize = 1000;
    const COLS: usize = 1000;
    let mut grid: Vec<usize> = vec![0; ROWS * COLS];

    for line in lines {
        if line.0 == line.2 {
            dbg!(line);
            for x in min(line.1, line.3)..=max(line.1, line.3) {
                grid[line.0 * ROWS + x] += 1;
            }
        } else if line.1 == line.3 {
            dbg!(line);
            for x in min(line.0, line.2)..=max(line.0, line.2) {
                grid[x * ROWS + line.1] += 1;
            }
        }
    }

    grid.iter().filter(|x| **x > 1).count()
}

fn approach(start: usize, target: usize) -> usize {
    if start < target {
        start + 1
    } else if start > target {
        start - 1
    } else {
        start
    }
}

fn day5_2(lines: &[Line]) -> usize {
    const ROWS: usize = 1000;
    const COLS: usize = 1000;
    let mut grid: Vec<usize> = vec![0; ROWS * COLS];

    for line in lines {
        dbg!(line);
        let target = (line.2, line.3);

        let mut current = (line.0, line.1);

        loop {
            //dbg!(current);
            grid[current.0 * ROWS + current.1] += 1;

            if current == target {
                break;
            }

            current = (approach(current.0, target.0), approach(current.1, target.1));
        }
    }

    grid.iter().filter(|x| **x > 1).count()
}

#[test]
fn day5_example() {
    let input = &[
        Line(0, 9, 5, 9),
        Line(8, 0, 0, 8),
        Line(9, 4, 3, 4),
        Line(2, 2, 2, 1),
        Line(7, 0, 7, 4),
        Line(6, 4, 2, 0),
        Line(0, 9, 2, 9),
        Line(3, 4, 1, 4),
        Line(0, 0, 8, 8),
        Line(5, 5, 8, 2),
    ];

    assert_eq!(day5(input), 5);
}

#[test]
fn day5_actual() {
    let input = DAY5_INPUT;

    assert_eq!(day5(input), 7142);
}

#[test]
fn day5_2_example() {
    let input = &[
        Line(0, 9, 5, 9),
        Line(8, 0, 0, 8),
        Line(9, 4, 3, 4),
        Line(2, 2, 2, 1),
        Line(7, 0, 7, 4),
        Line(6, 4, 2, 0),
        Line(0, 9, 2, 9),
        Line(3, 4, 1, 4),
        Line(0, 0, 8, 8),
        Line(5, 5, 8, 2),
    ];

    assert_eq!(day5_2(input), 12);
}

#[test]
fn day5_2_actual() {
    let input = DAY5_INPUT;

    assert_eq!(day5_2(input), 20012);
}

const DAY5_INPUT: &[Line] = &[
    Line(223, 805, 223, 548),
    Line(609, 164, 609, 503),
    Line(461, 552, 796, 552),
    Line(207, 361, 207, 34),
    Line(503, 879, 503, 946),
    Line(937, 52, 937, 268),
    Line(560, 652, 118, 652),
    Line(771, 103, 85, 789),
    Line(119, 156, 947, 984),
    Line(356, 634, 607, 634),
    Line(348, 812, 873, 287),
    Line(409, 490, 726, 490),
    Line(298, 790, 298, 454),
    Line(407, 543, 820, 130),
    Line(206, 89, 591, 89),
    Line(164, 709, 976, 709),
    Line(208, 921, 208, 131),
    Line(515, 209, 515, 745),
    Line(876, 639, 281, 44),
    Line(270, 453, 727, 910),
    Line(190, 417, 190, 755),
    Line(522, 726, 903, 726),
    Line(390, 651, 603, 864),
    Line(707, 549, 926, 330),
    Line(471, 869, 471, 561),
    Line(970, 735, 401, 735),
    Line(612, 624, 612, 88),
    Line(844, 879, 844, 453),
    Line(400, 38, 400, 350),
    Line(832, 225, 984, 225),
    Line(971, 642, 42, 642),
    Line(70, 862, 447, 485),
    Line(183, 79, 183, 708),
    Line(598, 700, 598, 287),
    Line(635, 195, 39, 195),
    Line(587, 362, 349, 362),
    Line(108, 88, 965, 945),
    Line(700, 299, 165, 299),
    Line(295, 824, 785, 334),
    Line(211, 284, 390, 105),
    Line(288, 326, 672, 710),
    Line(595, 231, 595, 679),
    Line(671, 576, 813, 718),
    Line(14, 845, 784, 75),
    Line(700, 129, 43, 129),
    Line(83, 913, 889, 107),
    Line(830, 596, 322, 596),
    Line(391, 450, 391, 779),
    Line(384, 32, 384, 430),
    Line(311, 948, 938, 321),
    Line(460, 288, 460, 392),
    Line(924, 602, 924, 595),
    Line(703, 458, 703, 475),
    Line(335, 953, 335, 195),
    Line(692, 314, 927, 314),
    Line(131, 433, 131, 737),
    Line(590, 771, 965, 771),
    Line(650, 13, 963, 13),
    Line(586, 904, 658, 976),
    Line(238, 824, 782, 824),
    Line(366, 45, 691, 370),
    Line(428, 758, 201, 758),
    Line(240, 545, 30, 545),
    Line(396, 154, 332, 154),
    Line(549, 307, 233, 307),
    Line(187, 240, 851, 904),
    Line(151, 135, 937, 921),
    Line(342, 850, 342, 156),
    Line(695, 200, 695, 754),
    Line(385, 880, 893, 372),
    Line(986, 966, 813, 966),
    Line(727, 661, 727, 402),
    Line(316, 937, 316, 797),
    Line(422, 235, 422, 282),
    Line(965, 684, 882, 684),
    Line(266, 222, 419, 69),
    Line(649, 843, 635, 857),
    Line(618, 84, 126, 576),
    Line(588, 822, 588, 636),
    Line(569, 142, 569, 607),
    Line(899, 479, 488, 890),
    Line(986, 52, 369, 52),
    Line(987, 478, 551, 914),
    Line(867, 951, 973, 845),
    Line(90, 401, 304, 401),
    Line(60, 836, 798, 836),
    Line(143, 675, 686, 675),
    Line(743, 974, 743, 305),
    Line(981, 899, 551, 469),
    Line(705, 430, 493, 430),
    Line(301, 366, 823, 366),
    Line(978, 712, 617, 712),
    Line(426, 805, 426, 345),
    Line(532, 855, 532, 54),
    Line(612, 143, 612, 133),
    Line(57, 52, 955, 950),
    Line(880, 50, 16, 914),
    Line(89, 908, 89, 214),
    Line(487, 867, 586, 867),
    Line(181, 285, 181, 470),
    Line(526, 666, 86, 226),
    Line(117, 704, 117, 961),
    Line(289, 101, 983, 795),
    Line(586, 429, 442, 429),
    Line(442, 869, 734, 869),
    Line(564, 479, 564, 382),
    Line(447, 486, 62, 101),
    Line(42, 218, 509, 218),
    Line(21, 890, 843, 68),
    Line(84, 978, 921, 141),
    Line(590, 960, 590, 934),
    Line(54, 949, 967, 36),
    Line(799, 39, 767, 39),
    Line(979, 232, 979, 628),
    Line(489, 482, 339, 482),
    Line(759, 473, 290, 942),
    Line(960, 958, 32, 30),
    Line(134, 180, 134, 864),
    Line(972, 981, 13, 22),
    Line(106, 385, 11, 385),
    Line(849, 454, 447, 454),
    Line(477, 385, 955, 863),
    Line(853, 180, 922, 180),
    Line(509, 274, 751, 32),
    Line(905, 295, 779, 295),
    Line(661, 629, 104, 629),
    Line(935, 117, 93, 959),
    Line(165, 372, 746, 953),
    Line(988, 141, 122, 141),
    Line(625, 621, 625, 406),
    Line(24, 710, 465, 710),
    Line(417, 468, 851, 34),
    Line(365, 285, 572, 285),
    Line(217, 164, 217, 214),
    Line(943, 439, 465, 439),
    Line(80, 102, 80, 717),
    Line(869, 19, 54, 834),
    Line(399, 480, 399, 458),
    Line(644, 826, 644, 911),
    Line(651, 189, 651, 687),
    Line(671, 946, 332, 607),
    Line(531, 417, 657, 417),
    Line(847, 350, 847, 112),
    Line(315, 733, 871, 177),
    Line(749, 118, 692, 118),
    Line(55, 616, 55, 894),
    Line(570, 307, 633, 307),
    Line(12, 964, 883, 93),
    Line(84, 299, 84, 185),
    Line(49, 187, 903, 187),
    Line(592, 40, 842, 40),
    Line(639, 381, 802, 544),
    Line(59, 61, 836, 61),
    Line(968, 51, 266, 753),
    Line(883, 373, 883, 130),
    Line(612, 45, 406, 45),
    Line(206, 698, 206, 823),
    Line(385, 685, 385, 46),
    Line(656, 338, 73, 921),
    Line(256, 794, 365, 903),
    Line(671, 247, 248, 247),
    Line(722, 509, 635, 422),
    Line(460, 783, 615, 783),
    Line(946, 980, 946, 129),
    Line(343, 780, 343, 723),
    Line(218, 371, 218, 856),
    Line(363, 809, 143, 589),
    Line(434, 739, 889, 739),
    Line(75, 71, 975, 971),
    Line(57, 253, 582, 778),
    Line(976, 237, 976, 148),
    Line(386, 866, 386, 544),
    Line(901, 797, 901, 630),
    Line(976, 706, 195, 706),
    Line(264, 420, 272, 428),
    Line(693, 72, 693, 379),
    Line(888, 832, 888, 490),
    Line(363, 900, 363, 350),
    Line(25, 312, 25, 58),
    Line(292, 307, 481, 307),
    Line(715, 393, 976, 132),
    Line(641, 450, 96, 450),
    Line(650, 38, 432, 38),
    Line(339, 97, 476, 97),
    Line(916, 24, 13, 927),
    Line(933, 934, 34, 35),
    Line(971, 367, 971, 919),
    Line(726, 310, 477, 559),
    Line(12, 984, 986, 10),
    Line(318, 531, 318, 72),
    Line(604, 979, 12, 387),
    Line(890, 39, 890, 213),
    Line(944, 954, 33, 43),
    Line(507, 830, 284, 607),
    Line(724, 111, 724, 242),
    Line(425, 912, 425, 445),
    Line(371, 903, 371, 634),
    Line(415, 314, 415, 509),
    Line(884, 849, 884, 454),
    Line(726, 647, 447, 926),
    Line(588, 463, 588, 426),
    Line(807, 453, 807, 593),
    Line(32, 449, 975, 449),
    Line(593, 757, 593, 607),
    Line(521, 850, 521, 139),
    Line(843, 478, 843, 317),
    Line(408, 834, 408, 455),
    Line(65, 241, 864, 241),
    Line(532, 138, 613, 138),
    Line(477, 239, 477, 676),
    Line(92, 400, 92, 935),
    Line(268, 104, 300, 104),
    Line(942, 20, 93, 869),
    Line(294, 134, 695, 134),
    Line(748, 477, 748, 311),
    Line(581, 879, 481, 879),
    Line(292, 57, 874, 639),
    Line(829, 787, 944, 787),
    Line(130, 780, 442, 780),
    Line(754, 435, 956, 435),
    Line(306, 659, 306, 491),
    Line(252, 612, 646, 612),
    Line(846, 949, 846, 924),
    Line(197, 888, 145, 836),
    Line(156, 790, 151, 790),
    Line(903, 305, 671, 73),
    Line(195, 79, 195, 40),
    Line(781, 67, 781, 635),
    Line(742, 743, 742, 280),
    Line(297, 42, 618, 42),
    Line(237, 151, 156, 151),
    Line(851, 930, 47, 126),
    Line(425, 368, 659, 134),
    Line(57, 890, 898, 49),
    Line(86, 62, 86, 445),
    Line(133, 499, 133, 604),
    Line(202, 567, 872, 567),
    Line(749, 578, 749, 804),
    Line(379, 379, 147, 379),
    Line(510, 474, 510, 388),
    Line(184, 115, 738, 115),
    Line(904, 613, 550, 613),
    Line(755, 649, 755, 305),
    Line(461, 306, 461, 547),
    Line(469, 124, 542, 124),
    Line(736, 218, 736, 968),
    Line(307, 662, 307, 188),
    Line(360, 970, 58, 668),
    Line(36, 267, 214, 267),
    Line(980, 330, 22, 330),
    Line(222, 972, 222, 178),
    Line(846, 774, 714, 774),
    Line(798, 837, 789, 837),
    Line(567, 258, 567, 502),
    Line(325, 582, 325, 976),
    Line(138, 386, 138, 691),
    Line(326, 878, 326, 386),
    Line(790, 276, 811, 276),
    Line(517, 522, 81, 86),
    Line(493, 567, 406, 567),
    Line(522, 370, 13, 370),
    Line(31, 697, 607, 121),
    Line(297, 524, 297, 320),
    Line(790, 681, 753, 681),
    Line(90, 961, 901, 150),
    Line(262, 46, 262, 68),
    Line(18, 26, 977, 985),
    Line(782, 381, 956, 381),
    Line(353, 740, 353, 595),
    Line(32, 448, 941, 448),
    Line(405, 254, 686, 254),
    Line(853, 57, 297, 613),
    Line(555, 209, 439, 209),
    Line(765, 679, 142, 56),
    Line(175, 903, 175, 685),
    Line(693, 653, 845, 653),
    Line(394, 108, 394, 901),
    Line(351, 108, 335, 108),
    Line(841, 83, 841, 716),
    Line(525, 608, 525, 496),
    Line(874, 32, 874, 214),
    Line(354, 760, 44, 760),
    Line(249, 330, 864, 945),
    Line(553, 377, 553, 944),
    Line(903, 374, 335, 374),
    Line(387, 34, 387, 86),
    Line(380, 331, 380, 124),
    Line(618, 520, 797, 520),
    Line(718, 169, 703, 169),
    Line(355, 184, 851, 184),
    Line(582, 570, 582, 313),
    Line(312, 952, 312, 460),
    Line(269, 70, 269, 197),
    Line(701, 907, 701, 768),
    Line(645, 417, 645, 548),
    Line(931, 532, 367, 532),
    Line(497, 361, 497, 348),
    Line(563, 642, 976, 642),
    Line(376, 504, 376, 448),
    Line(538, 945, 538, 773),
    Line(594, 886, 594, 281),
    Line(879, 558, 192, 558),
    Line(985, 68, 66, 987),
    Line(599, 420, 599, 41),
    Line(296, 318, 296, 132),
    Line(330, 619, 302, 619),
    Line(245, 137, 918, 810),
    Line(823, 798, 556, 531),
    Line(64, 201, 723, 860),
    Line(955, 365, 955, 829),
    Line(372, 976, 255, 859),
    Line(804, 962, 168, 962),
    Line(200, 442, 200, 97),
    Line(965, 964, 870, 869),
    Line(534, 158, 128, 564),
    Line(380, 739, 577, 542),
    Line(740, 391, 740, 651),
    Line(167, 177, 619, 177),
    Line(215, 449, 215, 330),
    Line(494, 612, 19, 137),
    Line(458, 634, 458, 257),
    Line(884, 817, 393, 326),
    Line(407, 291, 19, 679),
    Line(627, 173, 627, 570),
    Line(53, 93, 552, 592),
    Line(809, 363, 119, 363),
    Line(588, 418, 588, 764),
    Line(807, 131, 807, 834),
    Line(616, 61, 514, 61),
    Line(553, 642, 236, 325),
    Line(959, 553, 683, 553),
    Line(36, 754, 36, 830),
    Line(533, 293, 144, 293),
    Line(950, 780, 396, 780),
    Line(949, 878, 14, 878),
    Line(453, 180, 989, 180),
    Line(22, 46, 670, 694),
    Line(479, 206, 479, 552),
    Line(22, 53, 599, 53),
    Line(254, 964, 884, 334),
    Line(578, 813, 100, 813),
    Line(945, 247, 778, 80),
    Line(312, 978, 312, 518),
    Line(882, 225, 882, 967),
    Line(581, 683, 293, 395),
    Line(107, 540, 534, 967),
    Line(382, 946, 28, 946),
    Line(864, 743, 246, 743),
    Line(538, 558, 733, 753),
    Line(811, 436, 814, 436),
    Line(982, 33, 65, 950),
    Line(785, 829, 945, 829),
    Line(322, 471, 346, 471),
    Line(904, 528, 904, 669),
    Line(231, 471, 772, 471),
    Line(773, 490, 669, 386),
    Line(867, 482, 417, 32),
    Line(352, 856, 352, 478),
    Line(723, 355, 619, 355),
    Line(667, 922, 667, 247),
    Line(642, 386, 241, 386),
    Line(594, 35, 594, 580),
    Line(916, 723, 793, 723),
    Line(73, 774, 269, 970),
    Line(43, 273, 148, 168),
    Line(744, 637, 825, 637),
    Line(98, 30, 98, 383),
    Line(130, 277, 802, 277),
    Line(167, 122, 672, 627),
    Line(871, 866, 564, 559),
    Line(923, 475, 539, 859),
    Line(422, 714, 422, 946),
    Line(667, 950, 667, 640),
    Line(758, 181, 12, 927),
    Line(129, 927, 129, 288),
    Line(485, 661, 402, 661),
    Line(114, 573, 974, 573),
    Line(674, 779, 851, 779),
    Line(977, 184, 977, 143),
    Line(229, 937, 229, 138),
    Line(520, 887, 520, 512),
    Line(918, 329, 918, 990),
    Line(732, 41, 521, 41),
    Line(399, 245, 883, 729),
    Line(824, 618, 356, 618),
    Line(215, 218, 845, 848),
    Line(704, 34, 307, 431),
    Line(124, 166, 696, 738),
    Line(692, 749, 839, 749),
    Line(790, 637, 790, 598),
    Line(697, 396, 669, 396),
    Line(419, 140, 113, 446),
    Line(426, 738, 171, 738),
    Line(489, 494, 190, 793),
    Line(320, 301, 320, 398),
    Line(275, 809, 275, 717),
    Line(537, 703, 465, 703),
    Line(536, 450, 560, 450),
    Line(153, 927, 914, 166),
    Line(246, 692, 485, 453),
    Line(26, 179, 26, 554),
    Line(487, 678, 487, 696),
    Line(807, 719, 224, 719),
    Line(605, 920, 899, 920),
    Line(112, 262, 112, 765),
    Line(752, 898, 752, 429),
    Line(861, 103, 861, 477),
    Line(628, 505, 628, 248),
    Line(556, 293, 556, 276),
    Line(826, 682, 273, 129),
    Line(685, 324, 685, 692),
    Line(544, 410, 544, 678),
    Line(796, 633, 796, 950),
    Line(753, 843, 753, 936),
    Line(817, 40, 817, 600),
    Line(137, 941, 677, 401),
    Line(563, 457, 599, 457),
    Line(251, 644, 251, 67),
    Line(170, 792, 805, 792),
    Line(171, 486, 171, 877),
    Line(337, 481, 268, 412),
    Line(43, 158, 44, 158),
    Line(148, 610, 863, 610),
    Line(332, 765, 202, 765),
    Line(307, 637, 334, 637),
    Line(557, 380, 231, 54),
    Line(858, 76, 150, 784),
    Line(477, 329, 319, 329),
    Line(306, 608, 306, 38),
    Line(245, 42, 245, 929),
    Line(15, 786, 745, 786),
    Line(946, 321, 841, 321),
    Line(837, 281, 837, 762),
    Line(847, 391, 847, 840),
    Line(304, 52, 304, 299),
    Line(938, 122, 877, 122),
    Line(214, 347, 862, 347),
    Line(494, 540, 751, 540),
    Line(184, 29, 913, 758),
    Line(904, 12, 15, 901),
    Line(573, 23, 158, 23),
    Line(926, 603, 643, 603),
    Line(105, 506, 518, 506),
    Line(551, 917, 983, 917),
    Line(708, 33, 831, 33),
    Line(347, 173, 218, 44),
    Line(933, 175, 933, 781),
    Line(902, 556, 902, 812),
    Line(556, 485, 252, 789),
    Line(823, 807, 368, 352),
    Line(217, 744, 217, 470),
    Line(795, 455, 795, 783),
    Line(170, 944, 926, 188),
    Line(55, 655, 258, 655),
    Line(158, 57, 959, 858),
    Line(714, 823, 714, 550),
    Line(238, 18, 388, 18),
    Line(980, 985, 12, 17),
    Line(360, 596, 770, 596),
    Line(846, 684, 220, 58),
    Line(552, 107, 552, 974),
    Line(228, 552, 354, 552),
    Line(421, 41, 421, 103),
    Line(674, 475, 912, 475),
    Line(455, 626, 455, 683),
    Line(952, 841, 946, 841),
    Line(920, 792, 381, 253),
    Line(786, 918, 786, 175),
    Line(889, 859, 889, 24),
    Line(178, 604, 573, 209),
    Line(71, 621, 550, 621),
    Line(38, 36, 922, 920),
    Line(104, 690, 575, 690),
    Line(252, 883, 894, 241),
    Line(627, 107, 417, 107),
    Line(768, 913, 13, 158),
    Line(708, 337, 708, 407),
    Line(156, 941, 156, 297),
    Line(814, 653, 814, 829),
    Line(234, 920, 896, 920),
    Line(652, 170, 128, 170),
    Line(765, 825, 347, 825),
    Line(681, 901, 681, 112),
    Line(410, 301, 979, 301),
    Line(462, 681, 462, 726),
    Line(117, 957, 117, 693),
    Line(479, 948, 698, 948),
    Line(839, 965, 97, 223),
    Line(102, 189, 102, 366),
    Line(93, 798, 819, 72),
    Line(27, 336, 27, 655),
    Line(161, 635, 527, 269),
    Line(140, 272, 140, 336),
    Line(884, 915, 41, 72),
    Line(575, 563, 155, 563),
    Line(387, 601, 387, 597),
    Line(355, 186, 782, 613),
    Line(866, 435, 816, 435),
    Line(96, 161, 764, 161),
    Line(971, 29, 21, 979),
];
