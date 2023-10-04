-- Add migration script here

CREATE TABLE `todos`
(
    `id`          int(11)      NOT NULL,
    `title`       varchar(255) NOT NULL,
    `is_finished` tinyint(1)   NOT NULL DEFAULT 0
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

INSERT INTO `todos` (`id`, `title`, `is_finished`)
VALUES (1, 'First Todo', 0),
       (2, 'Second Todo', 1);

ALTER TABLE `todos`
    ADD PRIMARY KEY (`id`);

ALTER TABLE `todos`
    MODIFY `id` int(11) NOT NULL AUTO_INCREMENT,
    AUTO_INCREMENT = 3;
COMMIT;