-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Hôte : 127.0.0.1
-- Généré le : ven. 14 juin 2024 à 14:11
-- Version du serveur : 10.4.32-MariaDB
-- Version de PHP : 8.2.12

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Base de données : `blog_master`
--

-- --------------------------------------------------------

--
-- Structure de la table `article`
--

CREATE TABLE `article` (
  `id` int(11) NOT NULL,
  `titre` varchar(50) NOT NULL,
  `contenu` text DEFAULT NULL,
  `blog_id` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

--
-- Déchargement des données de la table `article`
--

INSERT INTO `article` (`id`, `titre`, `contenu`, `blog_id`) VALUES
(1, 'Purée au beurre', '(1) Ajoutez la purée (2) Ajoutez le beurre (3) Dégustez', 3),
(2, 'Opéra', 'Prendre du chocolat et....................................', 3),
(3, 'La voiture rouge', 'Une voiture rouge', 7),
(4, 'La voiture bleue', 'Une voiture bleue', 7),
(5, 'Article déjà existant', 'Article qui était la', 8),
(6, 'Nouvel article', 'fghujikolm', 8),
(7, 'fghjk', 'hjk', 8);

-- --------------------------------------------------------

--
-- Structure de la table `blog`
--

CREATE TABLE `blog` (
  `id` int(11) NOT NULL,
  `nom` varchar(50) DEFAULT NULL,
  `description` text NOT NULL,
  `galerie` tinyint(1) NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

--
-- Déchargement des données de la table `blog`
--

INSERT INTO `blog` (`id`, `nom`, `description`, `galerie`) VALUES
(1, 'Blog d\'informatique', 'Blog concernant l\'informatique en général', 0),
(3, 'Blog de Cuisine', 'Blog sur des recettes de cuisine', 0),
(4, 'Blog sans but', 'Un blog', 0),
(5, 'Blog test', 'b', 0),
(6, 'Le jeu drôle', 'Blog de jeux de rôles', 0),
(7, 'Vroum', 'Blog de voiture', 0),
(8, 'Blog de présentation', 'Blog utilisé pour la démo', 0),
(9, 'Nouveau Blog', 'Nouveau Blog', 0);

-- --------------------------------------------------------

--
-- Structure de la table `galerie`
--

CREATE TABLE `galerie` (
  `id` int(11) NOT NULL,
  `name` int(11) NOT NULL,
  `file` int(11) NOT NULL,
  `blog_id` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

-- --------------------------------------------------------

--
-- Structure de la table `message`
--

CREATE TABLE `message` (
  `id` int(11) NOT NULL,
  `username` varchar(50) NOT NULL,
  `article_id` int(11) NOT NULL,
  `time` varchar(50) NOT NULL,
  `texte` varchar(250) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Déchargement des données de la table `message`
--

INSERT INTO `message` (`id`, `username`, `article_id`, `time`, `texte`) VALUES
(3, 'Stevia', 4, '2024-06-13 23:01:20.181848800 +02:00', 'Test'),
(4, 'Stevia', 4, '2024-06-13 23:01:24.053806200 +02:00', 'test 2'),
(5, 'PasStevia', 4, '2024-06-13 23:02:27.277383900 +02:00', 'Hey !'),
(6, 'Steven', 5, '2024-06-14 13:22:38.875421 +02:00', 'Bonjour'),
(7, 'Jacopo', 5, '2024-06-14 13:23:07.985956500 +02:00', 'Hey :'),
(8, 'Jacopo', 5, '2024-06-14 13:23:24.060940100 +02:00', 'ça va ?');

--
-- Index pour les tables déchargées
--

--
-- Index pour la table `article`
--
ALTER TABLE `article`
  ADD PRIMARY KEY (`id`),
  ADD KEY `blog_id` (`blog_id`);

--
-- Index pour la table `blog`
--
ALTER TABLE `blog`
  ADD PRIMARY KEY (`id`);

--
-- Index pour la table `galerie`
--
ALTER TABLE `galerie`
  ADD PRIMARY KEY (`id`),
  ADD KEY `blog_id` (`blog_id`);

--
-- Index pour la table `message`
--
ALTER TABLE `message`
  ADD PRIMARY KEY (`id`);

--
-- AUTO_INCREMENT pour les tables déchargées
--

--
-- AUTO_INCREMENT pour la table `article`
--
ALTER TABLE `article`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT pour la table `blog`
--
ALTER TABLE `blog`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=10;

--
-- AUTO_INCREMENT pour la table `galerie`
--
ALTER TABLE `galerie`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT pour la table `message`
--
ALTER TABLE `message`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=9;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
